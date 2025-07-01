// TODO: Add the game struct and its methods

use crate::enums::card_types::CardType;
use crate::enums::game_state::GameState;
use crate::types::chat::Chat;
use crate::types::claim::Claim;
use serde::Deserialize;
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
#[derive(Deserialize)]
pub struct Game {
    /// Unique identifier for the game instance.
    pub id: Uuid,
    /// List of player IDs participating in the game.
    pub player_ids: Vec<Uuid>,
    /// ID of the player whose turn it is.
    pub which_player_turn: uuid::Uuid, // ID of the player whose turn it is
    /// Current state of the game, represented as a string.
    pub state: GameState,
    /// Timestamp when the game was created
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The round number of the game
    pub round: usize,
    /// Chat of the specific game
    pub chat: Chat,
    /// Changes after every round and is randomly selected.
    pub card_to_play: CardType,
    /// Vector of claims every player made
    pub claims: Vec<Claim>,
}

// TODO: Add the necessary method for the game handling

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
            id: Uuid::new_v4(),
            player_ids: Vec::new(),
            which_player_turn: Uuid::new_v4(),
            state: GameState::Starting, // Placeholder for actual game state
            started_at: chrono::Utc::now(),
            card_to_play: CardType::King,
            chat: Chat::new(),
            claims: vec![],
            round: 0,
        }
    }
}
