use db::{Db, DbLayer};
use heck::{ToLowerCamelCase, ToPascalCase};
use router::path::PathSegment;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod error;
mod router;
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

    let mut b = JsFunctionBuilder::default();
    app.visit(&mut b);

    std::fs::write("web/js/REST/paths.js", b.result).unwrap();

    let app = axum::Router::from(app);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    tracing::debug!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Default)]
pub struct JsFunctionBuilder {
    result: String,
}

impl router::PathVisitor<'_> for JsFunctionBuilder {
    fn visit(&mut self, name: &str, path: &router::path::Path<'_>) {
        let x = format!(
            "\n// {path}\nexport function {}({}) {{ return `{}` }}\n",
            name.to_lower_camel_case(),
            path.iter()
                .filter_map(|x| if let PathSegment::RegParam(x) = x {
                    Some(format!("{x}, "))
                } else {
                    None
                })
                .collect::<String>(),
            path.iter()
                .map(|x| match x {
                    PathSegment::Normal(cow) => format!("/{cow}"),
                    PathSegment::RegParam(cow) => format!("/${{{cow}}}"),
                })
                .chain(std::iter::once_with(|| "/".into()))
                .collect::<String>()
        );
        self.result.push_str(&x);
    }
}
