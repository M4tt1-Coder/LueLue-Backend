// TODO: Set up all necessary handler functions regarding serving  with the game instance
// TODO: Add the D1 database connection

use axum::{
    extract::Request,
    http::{self, StatusCode},
    Json,
};
use axum_macros::debug_handler;

use crate::types::game::Game;

/// Updates a game instance and modifies the database entries by using the provided id.
///
/// URL endpoint: /game/update
#[debug_handler]
pub async fn update_game(request: Request) -> Result<Json<Game>, StatusCode> {
    let body = request.body();

    Err(http::StatusCode::OK)
}
