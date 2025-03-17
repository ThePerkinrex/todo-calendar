use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        time::{Time, TimeData, TimeId},
    },
    error::AppError,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(all))
        .route("/", post(add))
        .route("/{id}", get(single))
        .route("/{id}", put(update))
        .route("/{id}", delete(delete_course))
}

async fn all(db: Db) -> Result<Json<Vec<Time>>, AppError> {
    Ok(Json(Time::get_all(&db).await?))
}

async fn add(db: Db, Json(data): Json<TimeData>) -> Result<Json<TimeId>, AppError> {
    Ok(Json(Time::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<TimeId>,
    Json(data): Json<TimeData>,
) -> Result<StatusCode, AppError> {
    let mut course = Time::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<TimeId>) -> Result<Json<Time>, AppError> {
    let course = Time::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<TimeId>) -> Result<StatusCode, AppError> {
    Time::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
