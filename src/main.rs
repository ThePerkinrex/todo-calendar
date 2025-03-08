use db::{Db, DbLayer};
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod error;
mod routes;

const ADDR: &str = "127.0.0.1:5010";

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        format!(
            "{}=debug,tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    });
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db = match Db::new().await {
        Ok(x) => x,
        Err(e) => {
            error!("Db error:\n{e}");
            panic!()
        }
    };
    // build our application with a single route
    let app = routes::router()
        .layer(DbLayer::new(db))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
