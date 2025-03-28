use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use axum_extra::extract::Query;

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAllPart, DbReadSingle, DbTable, DbUpdate,
        task::{Task, TaskData, TaskFilter, TaskId},
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

async fn all(db: Db, Query(filter): Query<TaskFilter>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = Task::get_all_for(&db, &filter).await?;
    // tasks.sort_by(|a, b| a..cmp(&b.timestamp));
    Ok(Json(tasks))
}

async fn add(db: Db, Json(data): Json<TaskData>) -> Result<Json<TaskId>, AppError> {
    Ok(Json(Task::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<TaskId>,
    Json(data): Json<TaskData>,
) -> Result<StatusCode, AppError> {
    let mut course = Task::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<TaskId>) -> Result<Json<Task>, AppError> {
    let course = Task::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<TaskId>) -> Result<StatusCode, AppError> {
    Task::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
