use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};

use crate::{
    db::{
        Db, DbCreate, DbDelete, DbReadAll, DbReadSingle, DbTable, DbUpdate,
        event::{Event, EventData, EventId},
    },
    error::AppError,
};

mod category;

pub fn router() -> Router {
    Router::new()
        .route("/", get(all))
        .route("/", post(add))
        .route("/{id}/", put(update))
        .route("/{id}/", get(single))
        .route("/{id}/", delete(delete_course))
        .nest("/category", category::router())
}

async fn all(db: Db) -> Result<Json<Vec<Event>>, AppError> {
    Ok(Json(Event::get_all(&db).await?))
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
