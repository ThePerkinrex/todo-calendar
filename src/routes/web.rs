use std::path::Path;

use tower_http::services::{ServeDir, ServeFile};

use crate::{path, router::Router};

pub fn get_pages<'a, P: AsRef<Path>>(path: P) -> std::io::Result<Router<'a>> {
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

                let path = {
                    let mut x = crate::router::path::Path::new();
                    if !name.is_empty() {
                        x.normal(name.clone());
                    }
                    x
                }; // format!("/{name}{}", if name.is_empty() {""}else{"/"});

                tracing::info!(
                    "Added page at {} with name {name} -> {path}",
                    f.path().display()
                );
                let mut r = Router::new().fallback_service(ServeDir::new(f.path()));
                if index_file.exists() && index_file.is_file() {
                    r = r.route_service("index", path!(/), ServeFile::new(index_file))
                }
                Ok(Some((path, name, r)))
            } else {
                Ok(None)
            }
        })
        .filter_map(Result::transpose)
        .try_fold(
            Router::new(),
            |r, x: std::io::Result<(crate::router::path::Path, String, Router)>| {
                let (path, name, router) = x?;
                if path.is_empty() {
                    Ok(r.merge(router))
                } else {
                    Ok(r.nest(name, path, router))
                }
            },
        )
}
