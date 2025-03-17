use axum::{Router, response::Redirect, routing::get};
use tower_http::services::ServeDir;

mod categories;
mod colors;
mod courses;
mod data;
mod states;
mod tasks;
mod times;
// mod deadlines;
// mod events;

// TODO Themes
pub fn router() -> Router {
    Router::new()
        .nest("/courses", courses::router())
        .nest("/colors", colors::router())
        .nest("/categories", categories::router())
        .nest("/states", states::router())
        .nest("/tasks", tasks::router())
        .nest("/times", times::router())
        // .nest("/deadlines", deadlines::router())
        // .nest("/events", events::router())
        .nest("/data", data::router())
        .route("/", get(async || Redirect::to("/index.html")))
        .fallback_service(ServeDir::new("web"))
}
