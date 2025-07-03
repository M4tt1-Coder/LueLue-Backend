// use statements
use axum::routing::{get, put};
use axum::Router;

use crate::handlers::game_handlers::update_game;

/// Router provider for the Axum application.
///
/// This module defines the router for the Axum application, setting up the routes
pub fn router() -> Router {
    Router::new()
        // game instance endpoints
        .route("/game/update", put(update_game))
}
