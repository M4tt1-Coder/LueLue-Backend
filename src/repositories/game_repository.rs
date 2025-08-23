use crate::{
    errors::database_query_error::DatabaseQueryError,
    repositories::{card_repository::CardRepository, claim_repository::ClaimsRepository, player_repository::PlayerRepository},
    types::{
        chat::Chat,
        claim::Claim,
        game::{Game, UpdateGameDTO},
        player::Player,
    },
};
use axum::{http::StatusCode, Json};
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
    /// - `game` - A reference to the `Game` instance containing updated information.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn update_game(
        &self,
        game_data: UpdateGameDTO,
        player_repo: &PlayerRepository<'_>,
        card_repo: &CardRepository<'_>
    ) -> Result<Game, DatabaseQueryError<UpdateGameDTO>> {
        let (query, bindings) = self.get_update_query_string_and_bindings(&game_data);

        let mut query_result = self
            .db
            .prepare(&query)
            .bind(&bindings)
            .unwrap()
            .first::<Game>(None)
            .await;

        // TODO: Handle relations like claims, chat with other queries
        
        match query_result {
            Ok(game) => match game {
                Some(mut updated_game) => {
                    updated_game.players = match self.update_players_in_game(&game_data, &player_repo, card_repo).await {
                        Ok(players) => players,
                        Err(err) => return Err(DatabaseQueryError::new(err.message, match err.received_data {
                            None => None,
                            Some(_) => Some(Json(game_data.clone()))
                        }, err.status_code))
                    };  

                    return Ok(updated_game);
                },
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
    /// A `Result` containing an `Game` struct object if the game is found, or a `DatabaseQueryError` if
    /// an error occurs.
    pub async fn get_game_by_id(
        &self,
        game_id: &str,
    ) -> Result<Game, DatabaseQueryError<Game>> {
        let query_result = self
            .db
            .prepare("SELECT * FROM games WHERE id = ?;")
            .bind(&[JsValue::from(game_id)])
            .unwrap()
            .first::<Game>(None)
            .await;

        match query_result {
            Ok(game) => match game {
                Some(game) => Ok(game),
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

    /// Deletes a game by its ID from the D1 database.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice representing the ID of the game to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn delete_game(&self, game_id: &str) -> Result<(), DatabaseQueryError<Game>> {
        let query_result = self
            .db
            .prepare("DELETE FROM games WHERE id = ?;")
            .bind(&[JsValue::from(game_id)])
            .unwrap()
            .run()
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    // ----- utility functions of the 'GameRepository' struct -----

    /// Combines all properties together that are directly stored in the 'games' table.
    ///
    /// Fields that weren't supposed to be updated aren't included.
    ///
    /// # Arguments
    ///
    /// - `game_data` -> DTO object which holds new data stored in the `games` table
    fn get_update_query_string_and_bindings(
        &self,
        game_data: &UpdateGameDTO,
    ) -> (String, Vec<JsValue>) {
        let mut output_query = "UPDATE games SET ".to_string();
        let mut output_bindings = vec![];

        // game state
        if let Some(state) = &game_data.state {
            output_query.push_str("state = ?, ");
            output_bindings.push(JsValue::from(state.index()));
        }

        // round number
        if let Some(round) = game_data.round_number {
            output_query.push_str("round_number = ?, ");
            output_bindings.push(JsValue::from(round));
        }

        // card to play
        if let Some(card) = &game_data.card_to_play {
            output_query.push_str("card_to_play = ?, ");
            output_bindings.push(JsValue::from(card.index()));
        }

        // which players turn it is
        if let Some(player) = &game_data.which_player_turn {
            output_query.push_str("which_player_turn = ?, ");
            output_bindings.push(JsValue::from(player));
        }

        output_query.truncate(output_query.len() - 2);
        output_query.push_str(" WHERE id = ? RETURNING *;");
        output_bindings.push(JsValue::from(game_data.id.clone()));

        (output_query, output_bindings)
    }

    /// Fetches all curent players of the game stored in the database and then determines which
    /// entities to delete or add.
    ///
    /// # Returns
    ///
    /// - List of `Player`, which was passed to the function.
    ///
    /// # Arguments
    ///
    /// - `game_data` -> DTO object containing the list players
    /// - `player_repo` -> Player database repository passed from the handler function
    async fn update_players_in_game(
        &self,
        game_data: &UpdateGameDTO,
        player_repo: &PlayerRepository<'_>,
        card_repo: &CardRepository<'_>
    ) -> Result<Vec<Player>, DatabaseQueryError<UpdateGameDTO>> {
        // just to make sure that the needed data was provided
        let new_players = match &game_data.players {
            None => {
                return Err(DatabaseQueryError { 
                    message: "Function was called with invalid data passed to it! A new list of players is mandatory!".to_string(), 
                    received_data: None, 
                    status_code: StatusCode::INTERNAL_SERVER_ERROR 
                });
            },
            Some(players) => {
                if players.len() == 0 {
                    return Err(DatabaseQueryError { 
                        message: "An empty list of players was provided! That's an invalid data input!".to_string(), 
                        received_data: None, 
                        status_code: StatusCode::BAD_REQUEST 
                    });
                }
                players
            }
        };

        // get all players first
        let all_current_players: Vec<Player> = match player_repo.get_all_players(Some(game_data.id.clone()), card_repo).await {
            Ok(players) => players,
            Err(err) => {
                return Err(DatabaseQueryError::new(
                    err.message,
                    match err.received_data {
                        None => None,
                        Some(_) => Some(Json(game_data.clone())),
                    },
                    err.status_code,
                ))
            }
        };

        // -> leave all entities that haven't changed
        // delete all players that are not in the updated list
        for player in all_current_players.clone() {
            match new_players.iter().find(|&p| p.id == player.id) {
                None => {
                    // delete the player
                    match player_repo.delete_player(&player.id).await {
                        Ok(_) => continue,
                        Err(err) => return Err(DatabaseQueryError { 
                            message: err.message, 
                            received_data: match err.received_data {
                                None => None,
                                Some(_) => Some(Json(game_data.clone()))
                            }, 
                            status_code: err.status_code 
                        })
                    };
                } 
                Some(_) => continue
            }
        }

        // add new entries
        for player in new_players {
            match all_current_players.iter().find(|&p| p.id == player.id) {
                None => {
                    match player_repo.add_player(player.clone()).await {
                        Ok(_) => continue,
                        Err(err) => return Err(DatabaseQueryError { 
                            message: err.message, 
                            received_data: match err.received_data {
                                None => None,
                                Some(_) => Some(Json(game_data.clone()))
                            }, 
                            status_code: err.status_code 
                        })
                    }
                }
                Some(_) => continue
            }
        } 


        // return modified list of players
        Ok(all_current_players)
    }

    // TODO: Implement the method to update all claims of a game

    /// 
    async fn update_claims_of_game(&self, game_data: &UpdateGameDTO, claims_repo: &ClaimsRepository<'_>) -> Result<Vec<Claim>, DatabaseQueryError<UpdateGameDTO>> {}
}
