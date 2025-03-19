use axum::Router;
use tower_http::services::ServeDir;
use web::get_pages;

mod categories;
mod colors;
mod courses;
mod data;
mod states;
mod tasks;
mod times;
mod web;
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
        .nest_service("/lib", ServeDir::new("web/lib/"))
        .nest_service("/js", ServeDir::new("web/js/"))
        .nest_service("/styles", ServeDir::new("web/styles/"))
        // .route("/", get(async || Redirect::to("/index.html")))
        .merge(get_pages("web/pages/").unwrap())
}
