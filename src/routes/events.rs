use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use axum_extra::extract::Query;

use crate::{
    db::{
        event::{Event, EventData, EventFilter, EventId}, Db, DbCreate, DbDelete, DbReadAllPart, DbReadSingle, DbTable, DbUpdate
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

async fn all(db: Db, Query(filter): Query<EventFilter>) -> Result<Json<Vec<Event>>, AppError> {
    Ok(Json(Event::get_all_for(&db, &filter).await?))
}

async fn add(db: Db, Json(data): Json<EventData>) -> Result<Json<EventId>, AppError> {
    Ok(Json(Event::new(&db, data).await?.id()))
}

async fn update(
    db: Db,
    Path(id): Path<EventId>,
    Json(data): Json<EventData>,
) -> Result<StatusCode, AppError> {
    let mut course = Event::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    course.set(data);
    course.save(&db).await?;
    Ok(StatusCode::OK)
}

async fn single(db: Db, Path(id): Path<EventId>) -> Result<Json<Event>, AppError> {
    let course = Event::get(&db, &id).await?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(course))
}

async fn delete_course(db: Db, Path(id): Path<EventId>) -> Result<StatusCode, AppError> {
    Event::delete_static(&db, &id).await?;
    Ok(StatusCode::OK)
}
