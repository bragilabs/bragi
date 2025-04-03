use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/tracks", get(get_tracks))
}

async fn get_tracks() -> &'static str {
    return "Hello, tracks!";
}