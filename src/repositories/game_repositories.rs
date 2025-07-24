// TODO: Implement the game repositories for the interaction with the database

use std::error::Error;

use crate::{errors::database_query_error::DatabaseQueryError, types::game::Game};
use wasm_bindgen::JsValue;
use worker::D1Database;

/// Represents a repository for managing game data in the D1 database.
///
/// This repository provides methods to interact with the game data stored in the D1 database,
/// including creating, updating, and retrieving game instances.
///
/// # Properties
///
/// `db`: An instance of `D1Database` that provides access to the D1 database.
#[derive(Clone)]
pub struct GameRepository<'a> {
    /// The D1 database instance used for accessing game data.
    db: &'a D1Database,
}

impl<'a> GameRepository<'a> {
    /// Creates a new `GameRepository` instance with the provided D1 database.
    ///
    /// # Arguments
    ///
    /// * `db` - An instance of `D1Database` to be used for database operations.
    ///
    /// # Returns
    ///
    /// A new `GameRepository` instance.
    pub fn new(db: &'a D1Database) -> Self {
        GameRepository { db }
    }

    // pub fn db(&self) -> &D1Database {
    //    &self.db
    // }

    /// Adds a new game to the D1 database.
    ///
    /// # Arguments
    ///
    /// * `game` - A reference to the `Game` instance to be added to the database.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn add_game(&self, game: Game) -> Result<Game, DatabaseQueryError<Game>> {
        let added_game = self
            .db
            .prepare(
                "INSERT INTO games (id, started_at, round_number, state, which_players_turn, card_to_play) 
                    VALUES (1?, 2?, 3?, 4?, 5?, 6?) RETURNING *;",
            )
            .bind(&[
                JsValue::from(game.id),
                JsValue::from(game.started_at),
                JsValue::from(game.round_number),
                JsValue::from(game.state.index()),
                JsValue::from(game.which_player_turn),
                JsValue::from(game.card_to_play.index()),
            ]).unwrap().first::<Game>(None).await;

        match added_game {
            Ok(game) => match game {
                Some(game) => Ok(game),
                None => Err(DatabaseQueryError::new(
                    "Failed to add game to the database".to_string(),
                    None,
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Updates an existing game in the D1 database.
    pub fn update_game(&self, game: Game) -> Result<(), String> {
        todo!("Implement the logic to update a game in the D1 database");
    }

    /// Retrieves a game by its ID from the D1 database.
    pub fn get_game_by_id(&self, game_id: &str) -> Result<Option<Game>, String> {
        todo!("Implement the logic to retrieve a game by its ID from the D1 database");
    }
}
