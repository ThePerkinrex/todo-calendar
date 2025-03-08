use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        deadline::{DeadlineCategory, DeadlineCategoryData, DeadlineCategoryId},
    },
    error::AppError,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(all))
        .route("/", post(add))
        .route("/{id}/", put(update))
        .route("/{id}/", get(single))
        .route("/{id}/", delete(delete_course))
}

async fn all(db: Db) -> Result<Json<Vec<DeadlineCategory>>, AppError> {
    Ok(Json(DeadlineCategory::get_all(&db).await?))
}

async fn add(
    db: Db,
    Json(data): Json<DeadlineCategoryData>,
) -> Result<Json<DeadlineCategoryId>, AppError> {
    Ok(Json(DeadlineCategory::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<DeadlineCategoryId>,
    Json(data): Json<DeadlineCategoryData>,
) -> Result<StatusCode, AppError> {
    let mut course = DeadlineCategory::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(
    db: Db,
    Path(id): Path<DeadlineCategoryId>,
) -> Result<Json<DeadlineCategory>, AppError> {
    let course = DeadlineCategory::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<DeadlineCategoryId>) -> Result<StatusCode, AppError> {
    DeadlineCategory::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
