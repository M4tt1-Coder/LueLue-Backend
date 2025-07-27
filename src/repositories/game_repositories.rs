// TODO: Implement the game repositories for the interaction with the database

use crate::{
    errors::database_query_error::DatabaseQueryError,
    types::{chat::Chat, claim::Claim, game::Game, player::Player},
};
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
    ///
    /// # Arguments
    ///
    /// * `game` - A reference to the `Game` instance containing updated information.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn update_game(&self, game: Game) -> Result<Game, DatabaseQueryError<Game>> {
        let query_result = self.db
            .prepare(
                "UPDATE games SET started_at = 1?, round_number = 2?, state = 3?, which_players_turn = 4?, card_to_play = 5?
                    WHERE id = 6? 
                    RETURNING *;",
            )
            .bind(&[
                JsValue::from(game.started_at),
                JsValue::from(game.round_number),
                JsValue::from(game.state.index()),
                JsValue::from(game.which_player_turn),
                JsValue::from(game.card_to_play.index()),
                JsValue::from(game.id),
            ])
            .unwrap()
            .first::<Game>(None).await;

        match query_result {
            Ok(game) => match game {
                Some(updated_game) => Ok(updated_game),
                None => Err(DatabaseQueryError::new(
                    "Failed to update game in the database".to_string(),
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

    /// Retrieves a game by its ID from the D1 database.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice representing the ID of the game to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<Game>` if the game is found, or a `DatabaseQueryError` if
    /// an error occurs.
    pub async fn get_game_by_id(
        &self,
        game_id: &str,
    ) -> Result<Option<Game>, DatabaseQueryError<Game>> {
        let query_result = self
            .db
            .prepare("SELECT * FROM games WHERE id = ?;")
            .bind(&[JsValue::from(game_id)])
            .unwrap()
            .first::<Game>(None)
            .await;

        match query_result {
            Ok(game) => match game {
                Some(game) => Ok(Some(game)),
                None => Err(DatabaseQueryError::new(
                    "Game not found".to_string(),
                    None,
                    axum::http::StatusCode::NOT_FOUND,
                )),
            },
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Retrieves all games from the D1 database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Game` instances if successful, or a `DatabaseQueryError`
    /// if an error occurs.
    pub async fn get_all_games(&self) -> Result<Vec<Game>, DatabaseQueryError<Game>> {
        let query_result = self
            .db
            .prepare("SELECT * FROM games;")
            .bind(&[])
            .unwrap()
            .all()
            .await;

        match query_result {
            Ok(collected_games) => {
                let mut output: Vec<Game> = collected_games.results::<Game>().unwrap();

                if output.is_empty() {
                    Err(DatabaseQueryError::new(
                        "No games found".to_string(),
                        None,
                        axum::http::StatusCode::NOT_FOUND,
                    ))
                } else {
                    // TODO: Replace the database query with repository functions for each
                    // structure

                    // Retrieve all other necessary game data (players, claims, chat) here
                    output.iter_mut().map(async |game| {
                        // players
                        let players = self
                            .db
                            .prepare("SELECT * FROM players WHERE game_id = ?;")
                            .bind(&[JsValue::from(game.id.clone())])
                            .unwrap()
                            .all()
                            .await
                            .unwrap()
                            .results::<Player>()
                            .unwrap();

                        // Assign players to the game
                        game.players = players;

                        // claims
                        let claims = self
                            .db
                            .prepare("SELECT * FROM claims WHERE game_id = ?;")
                            .bind(&[JsValue::from(game.id.clone())])
                            .unwrap()
                            .all()
                            .await
                            .unwrap()
                            .results::<Claim>()
                            .unwrap();

                        // Assign claims to the game
                        game.claims = claims;

                        // Retrieve chat for the game
                        let chat = self
                            .db
                            .prepare("SELECT * FROM chats WHERE game_id = ?;")
                            .bind(&[JsValue::from(game.id.clone())])
                            .unwrap()
                            .first::<Chat>(None)
                            .await
                            .unwrap();
                        // Assign chat to the game
                        game.chat = chat.unwrap_or_default();
                    });

                    Ok(output)
                }
            }

            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}
