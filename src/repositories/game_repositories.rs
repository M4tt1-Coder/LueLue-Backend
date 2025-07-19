// TODO: Implement the game repositories for the interaction with the database

use worker::D1Database;

use crate::types::game::Game;

/// Represents a repository for managing game data in the D1 database.
///
/// This repository provides methods to interact with the game data stored in the D1 database,
/// including creating, updating, and retrieving game instances.
///
/// # Properties
///
/// `db`: An instance of `D1Database` that provides access to the D1 database.
pub struct GameRepository {
    /// The D1 database instance used for accessing game data.
    db: D1Database,
}

impl GameRepository {
    /// Creates a new `GameRepository` instance with the provided D1 database.
    ///
    /// # Arguments
    ///
    /// * `db` - An instance of `D1Database` to be used for database operations.
    ///
    /// # Returns
    ///
    /// A new `GameRepository` instance.
    pub fn new(db: D1Database) -> Self {
        GameRepository { db }
    }

    // pub fn db(&self) -> &D1Database {
    //    &self.db
    // }

    // TODO: Add a custom error type for database operations

    /// Adds a new game to the D1 database.
    pub fn add_game(&self, game: &Game) -> Result<(), String> {
        todo!("Implement the logic to add a game to the D1 database");
    }

    /// Updates an existing game in the D1 database.
    pub fn update_game(&self, game: &Game) -> Result<(), String> {
        todo!("Implement the logic to update a game in the D1 database");
    }

    /// Retrieves a game by its ID from the D1 database.
    pub fn get_game_by_id(&self, game_id: &str) -> Result<Option<Game>, String> {
        todo!("Implement the logic to retrieve a game by its ID from the D1 database");
    }
}
