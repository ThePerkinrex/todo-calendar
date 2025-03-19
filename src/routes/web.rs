use std::path::Path;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn get_pages<P: AsRef<Path>>(path: P) -> std::io::Result<Router> {
    std::fs::read_dir(path)?
        .map(|f| {
            let f = f?;
            if f.file_type()?.is_dir() {
                let rename_file = f.path().join("path.txt");
                let index_file = f.path().join("index.html");
                let name = if rename_file.exists() && rename_file.is_file() {
                    std::fs::read_to_string(rename_file)?
                } else {
                    f.file_name().to_string_lossy().to_string()
                };
                let name = format!("/{name}");

                tracing::info!("Added page at {} with name {name}", f.path().display());
                let mut r = Router::new().fallback_service(ServeDir::new(f.path()));
                if index_file.exists() && index_file.is_file() {
                    r = r.route_service("/", ServeFile::new(index_file))
                }
                Ok(Some((name, r)))
            } else {
                Ok(None)
            }
        })
        .filter_map(Result::transpose)
        .try_fold(Router::new(), |r, x: std::io::Result<(String, Router)>| {
            let (name, router) = x?;
            if name == "/" {
                Ok(r.merge(router))
            } else {
                Ok(r.nest(&name, router))
            }
        })
}
