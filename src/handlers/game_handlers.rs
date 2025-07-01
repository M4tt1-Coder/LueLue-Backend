use axum::{http::StatusCode, Json};

use crate::types::game::Game;

pub async fn update_game() -> Result<Json<Game>, StatusCode> {
    return;
}
