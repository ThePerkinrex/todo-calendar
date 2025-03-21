use crate::{path, router::Router};
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

pub fn api<'a>() -> Router<'a> {
    Router::new()
        .nest("courses", path!(/ courses ), courses::router())
        .nest("colors", path!(/ colors), colors::router())
        .nest("categories", path!(/ categories), categories::router())
        .nest("states", path!(/ states), states::router())
        .nest("tasks", path!(/ tasks), tasks::router())
        .nest("times", path!(/ times), times::router())
        // .nest("/deadlines", deadlines::router())
        // .nest("/events", events::router())
        .nest("data", path!(/ data), data::router())
}

// TODO Themes
pub fn router<'a>() -> Router<'a> {
    Router::new()
        .nest("api v1", path!(/api/v1), api())
        .nest_service("lib", path!(/lib), ServeDir::new("web/lib/"))
        .nest_service("js", path!(/js), ServeDir::new("web/js/"))
        .nest_service("styles", path!(/styles), ServeDir::new("web/styles/"))
        // .route("/", get(async || Redirect::to("/index.html")))
        .merge(get_pages("web/pages/").unwrap())
}
