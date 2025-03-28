use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        course::{Course, CourseData, CourseId},
    },
    error::AppError,
    path,
    router::Router,
};

pub fn router<'a>() -> Router<'a> {
    Router::new()
        .route("all", path!(/), get(all))
        .route("post", path!(/), post(add))
        .route("get", path!(/{id}), get(single))
        .route("put", path!(/{id}), put(update))
        .route("delete", path!(/{id}), delete(delete_course))
}

async fn all(db: Db) -> Result<Json<Vec<Course>>, AppError> {
    Ok(Json(Course::get_all(&db).await?))
}

async fn add(db: Db, Json(data): Json<CourseData>) -> Result<Json<CourseId>, AppError> {
    Ok(Json(Course::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<CourseId>,
    Json(data): Json<CourseData>,
) -> Result<StatusCode, AppError> {
    let mut course = Course::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<CourseId>) -> Result<Json<Course>, AppError> {
    let course = Course::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<CourseId>) -> Result<StatusCode, AppError> {
    Course::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
