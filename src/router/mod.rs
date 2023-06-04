use crate::controllers::{root, ping};
use axum::{ routing::get, Router };

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/ping", get(ping))
}

