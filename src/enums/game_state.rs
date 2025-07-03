use serde::{Deserialize, Serialize};

/// Represents the current state of the game.
///
/// This enum defines the possible states a game can be in, such as:
///
/// - `InProgress`: The game is currently being played.
///
/// - `Ended`: The game has concluded.
/// - `WaitingForPlayers`: The game is waiting for players to join.
/// - `Starting`: The game is in the process of starting, preparing for the first turn.
///
/// Each variant represents a distinct phase in the lifecycle of a game, allowing for clear
/// management and transitions between states.
#[derive(Deserialize, Serialize)]
pub enum GameState {
    /// The game is currently in progress.
    InProgress,
    /// The game has ended.
    Ended,
    /// The game is waiting for players to join.
    WaitingForPlayers,
    /// The game is starting, preparing for the first turn.
    Starting,
}

impl GameState {
    /// Returns a string representation of the game state.
    ///
    /// # Returns
    /// A string slice representing the current game state.
    pub fn as_str(&self) -> &str {
        match self {
            GameState::InProgress => "In Progress",
            GameState::Ended => "Ended",
            GameState::WaitingForPlayers => "Waiting for Players",
            GameState::Starting => "Starting",
        }
    }
}
