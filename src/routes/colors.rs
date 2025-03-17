use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        color::{Color, ColorData, ColorId},
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

async fn all(db: Db) -> Result<Json<Vec<Color>>, AppError> {
    Ok(Json(Color::get_all(&db).await?))
}

async fn add(db: Db, Json(data): Json<ColorData>) -> Result<Json<ColorId>, AppError> {
    Ok(Json(Color::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<ColorId>,
    Json(data): Json<ColorData>,
) -> Result<StatusCode, AppError> {
    let mut course = Color::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<ColorId>) -> Result<Json<Color>, AppError> {
    let course = Color::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<ColorId>) -> Result<StatusCode, AppError> {
    Color::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
