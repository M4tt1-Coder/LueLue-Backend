// use statements
use axum::routing::{get, put};
use axum::Router;

use crate::handlers::game_handlers::update_game;
use crate::repositories::game_repository::GameRepository;

/// Application state for the Axum application.
///
/// This module defines the application state that will be shared across the Axum application.
///
/// # Properties
///     
/// `db`: An instance of `D1Database` that provides access to the D1 database.:w
///
#[derive(Clone)]
pub struct AppState {
    // Add application state properties here, e.g., database connection, configuration, etc.
    // For example:
    // pub db: D1Database,
    pub game_repository: GameRepository,
}

/// Router provider for the Axum application.
///
/// This module defines the router for the Axum application, setting up the routes
pub fn router(app_state: AppState) -> Router {
    Router::new()
        // game instance endpoints
        .route("/game/update", put(update_game))
        .with_state(app_state)
}
