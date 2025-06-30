// use statements
use crate::root;
use axum::routing::get;
use axum::Router;

/// Router provider for the Axum application.
///
/// This module defines the router for the Axum application, setting up the routes
pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/game/status", get(root))
}
