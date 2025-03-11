use std::{env, path::Path};

use axum::{
    Extension,
    body::Body,
    extract::FromRequestParts,
    http::{Request, request::Parts},
};
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool, migrate::Migrator};
use thiserror::Error;
use tower::{Layer, Service};

pub mod course;
pub mod deadline;
pub mod event;

#[derive(Debug, Error)]
pub enum GetPoolError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    Env(#[from] env::VarError),
}

#[derive(Debug, Clone)]
pub struct Db {
    pub(self) pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new() -> Result<Self, GetPoolError> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

        Migrator::new(Path::new("./migrations"))
            .await?
            .run(&pool)
            .await?;
        Ok(Self { pool })
    }
}

// impl Deref for Db {
// 	type Target = Pool<Sqlite>;

// 	fn deref(&self) -> &Self::Target {
// 		&self.pool
// 	}
// }

impl<S: Send + Sync + 'static> FromRequestParts<S> for Db {
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = <Extension<Self> as FromRequestParts<S>>::Rejection;

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Extension::from_request_parts(parts, state)
            .await
            .map(|Extension(d)| d)
    }
}

#[derive(Debug, Clone)]
pub struct DbLayer {
    db: Db,
}

impl DbLayer {
    pub const fn new(db: Db) -> Self {
        Self { db }
    }
}

impl<S> Layer<S> for DbLayer {
    type Service = DbService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DbService {
            db: self.db.clone(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbService<Inner> {
    db: Db,
    inner: Inner,
}

impl<Inner: Service<Request<Body>>> Service<Request<Body>> for DbService<Inner> {
    type Response = Inner::Response;

    type Error = Inner::Error;

    type Future = Inner::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        req.extensions_mut().insert(self.db.clone());
        self.inner.call(req)
    }
}

pub trait IdPart<Table>
where
    Table: DbTable,
{
}

pub trait DbTable: Sized {
    type Id;
    type Data;

    fn id(&self) -> Self::Id;
    fn data(&self) -> Self::Data;
}

pub trait DbCreate: DbTable {
    async fn new(db: &Db, data: Self::Data) -> sqlx::Result<Self>;
}

#[allow(unused)]
pub trait DbCreateWithId: DbTable {
    async fn new(db: &Db, id: Self::Id, data: Self::Data) -> sqlx::Result<Self>;
}

pub trait DbReadSingle: DbTable {
    async fn get(db: &Db, id: &Self::Id) -> sqlx::Result<Option<Self>>;
}

pub trait DbReadAll: DbTable {
    async fn get_all(db: &Db) -> sqlx::Result<Vec<Self>>;
}

pub trait DbReadAllPart<Id>: DbTable
where
    Id: IdPart<Self>,
{
    async fn get_all_for(db: &Db, id_part: &Id) -> sqlx::Result<Vec<Self>>;
}

pub trait DbUpdate: DbTable {
    async fn save(&self, db: &Db) -> sqlx::Result<()>;
}

pub trait DbDelete: DbTable {
    // async fn delete(self, db: &Db) -> sqlx::Result<()>;
    async fn delete_static(db: &Db, id: &Self::Id) -> sqlx::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(transparent)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct DateTime(chrono::DateTime<Utc>);

impl From<NaiveDateTime> for DateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self(Utc.from_utc_datetime(&value))
    }
}
