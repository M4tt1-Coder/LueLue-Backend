use std::fmt::{Debug, Display};

use crate::enums::game_state::GameState;
use crate::errors::application_error::ErrorObject;
use crate::types::chat::Chat;
use crate::types::claim::Claim;
use crate::{enums::card_types::CardType, types::player::Player};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// constants
/// The maximum number of players allowed in a game.
const MAX_PLAYERS: usize = 5;

/// Global struct representing a game in the system.k
///
/// Can be identified by its unique ID.
///
/// This struct is used to manage game instances and their associated data.
///
/// Holds information about the state of the game, such as players, scores, and other relevant
/// details.
#[derive(Deserialize, Serialize)]
pub struct Game {
    /// Unique identifier for the game instance.
    pub id: String,
    /// List of player IDs participating in the game.
    pub players: Vec<Player>,
    /// ID of the player whose turn it is.
    pub which_player_turn: String, // ID of the player whose turn it is
    /// Current state of the game, represented as a string.
    pub state: GameState,
    /// Timestamp when the game was created
    pub started_at: String,
    /// The round number of the game
    pub round_number: usize,
    /// Chat of the specific game
    pub chat: Chat,
    /// Changes after every round and is randomly selected.
    pub card_to_play: CardType,
    /// Vector of claims every player made
    pub claims: Vec<Claim>,
}

// TODO: Add the necessary methods for the game handling

impl Default for Game {
    /// Provides a default implementation for the `Game` struct.
    fn default() -> Self {
        Game::new()
    }
}

impl Game {
    /// Creates a new instance of the `Game` struct with a unique ID and default values.
    ///
    /// # Returns
    /// A new `Game` instance with a generated ID, an empty player list, and a default game state.
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use your_crate::game::Game;
    /// let game = Game::new();
    /// assert_eq!(game.id, Uuid::new_v4());
    /// assert!(game.player_ids.is_empty());
    /// assert_eq!(game.game_state, "initialized");
    /// ```
    pub fn new() -> Self {
        Game {
            id: Uuid::new_v4().to_string(),
            players: vec![],
            which_player_turn: String::new(),
            state: GameState::Starting, // Placeholder for actual game state
            started_at: chrono::Utc::now().to_string(),
            card_to_play: CardType::King,
            chat: Chat::new(),
            claims: vec![],
            round_number: 0,
        }
    }
}

// ----- Implementation 'ErrorObject' for 'Game' -----

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Game ID: {}, Players Number: {}, State: {}, Started At: {}, Round Number: {}",
            self.id,
            self.players.len(),
            self.state,
            self.started_at,
            self.round_number
        )
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Game {{ id: {}, players: {:?}, which_player_turn: {}, state: {:?}, started_at: {}, round_number: {}, card_to_play: {:?}, claims: {:?} }}",
            self.id,
            self.players,
            self.which_player_turn,
            self.state,
            self.started_at,
            self.round_number,
            self.card_to_play,
            self.claims
        )
    }
}

impl<'a> ErrorObject<'a> for Game {}
