use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        deadline::{Deadline, DeadlineData, DeadlineId},
    },
    error::AppError,
};

mod category;

pub fn router() -> Router {
    Router::new()
        .route("/", get(all))
        .route("/", post(add))
        .route("/{id}", put(update))
        .route("/{id}", get(single))
        .route("/{id}", delete(delete_course))
        .nest("/category", category::router())
}

async fn all(db: Db) -> Result<Json<Vec<Deadline>>, AppError> {
    Ok(Json(Deadline::get_all(&db).await?))
}

async fn add(db: Db, Json(data): Json<DeadlineData>) -> Result<Json<DeadlineId>, AppError> {
    Ok(Json(Deadline::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<DeadlineId>,
    Json(data): Json<DeadlineData>,
) -> Result<StatusCode, AppError> {
    let mut course = Deadline::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<DeadlineId>) -> Result<Json<Deadline>, AppError> {
    let course = Deadline::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<DeadlineId>) -> Result<StatusCode, AppError> {
    Deadline::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}