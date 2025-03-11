use axum::{Router, response::Redirect, routing::get};
use tower_http::services::ServeDir;

mod courses;
mod data;
mod deadlines;
mod events;

pub fn router() -> Router {
    Router::new()
        .nest("/courses", courses::router())
        .nest("/deadlines", deadlines::router())
        .nest("/events", events::router())
        .nest("/data", data::router())
        .route("/", get(async || Redirect::to("/index.html")))
        .fallback_service(ServeDir::new("web"))
}
