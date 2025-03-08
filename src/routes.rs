use axum::Router;

mod courses;
mod deadlines;
mod events;

pub fn router() -> Router {
    Router::new()
        .nest("/courses", courses::router())
        .nest("/deadlines", deadlines::router())
        .nest("/events", events::router())
}
