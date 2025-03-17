use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        category::{Category, CategoryData, CategoryId},
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

async fn all(db: Db) -> Result<Json<Vec<Category>>, AppError> {
    Ok(Json(Category::get_all(&db).await?))
}

async fn add(db: Db, Json(data): Json<CategoryData>) -> Result<Json<CategoryId>, AppError> {
    Ok(Json(Category::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<CategoryId>,
    Json(data): Json<CategoryData>,
) -> Result<StatusCode, AppError> {
    let mut course = Category::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<CategoryId>) -> Result<Json<Category>, AppError> {
    let course = Category::get(&db, &id)
        .await?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<CategoryId>) -> Result<StatusCode, AppError> {
    Category::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
