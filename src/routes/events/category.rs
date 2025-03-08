use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        event::{EventCategory, EventCategoryData, EventCategoryId},
    },
    error::AppError,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(all))
        .route("/", post(add))
        .route("/{id}", put(update))
        .route("/{id}", get(single))
        .route("/{id}", delete(delete_course))
}

async fn all(db: Db) -> Result<Json<Vec<EventCategory>>, AppError> {
    Ok(Json(EventCategory::get_all(&db).await?))
}

async fn add(
    db: Db,
    Json(data): Json<EventCategoryData>,
) -> Result<Json<EventCategoryId>, AppError> {
    Ok(Json(EventCategory::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<EventCategoryId>,
    Json(data): Json<EventCategoryData>,
) -> Result<StatusCode, AppError> {
    let mut course = EventCategory::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<EventCategoryId>) -> Result<Json<EventCategory>, AppError> {
    let course = EventCategory::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<EventCategoryId>) -> Result<StatusCode, AppError> {
    EventCategory::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
