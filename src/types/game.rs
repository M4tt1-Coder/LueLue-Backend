use std::fmt::{Debug, Display};

use crate::enums::game_state::GameState;
use crate::errors::application_error::ErrorObject;
use crate::errors::process_error::ProcessError;
use crate::types::chat::Chat;
use crate::types::claim::Claim;
use crate::utils::game_service::select_new_card_to_be_played;
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
    ///
    /// This property is static.
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

/// DTO type for the purpose of updating a game entry.
///
/// Just the ID of a Game instance is needed every other property can be empty.
///
/// # Props
///
/// - `id` -> Identifier of the Game instance; can't be null
/// - `players` -> List of new players
/// - `which_player_turn` -> New id of the player who's turn it is to make a claim
/// - `state` -> Editted state of a Game
/// - `round_number` -> New round number of a Game
/// - `chat` -> Potentially new chat instance
/// - `card_to_play` -> Changes after every made round
/// - `claims` -> List of claims in the current round
#[derive(Deserialize, Debug)]
pub struct UpdateGameDTO {
    /// Identifier of the game is always needed.
    pub id: String,
    /// Optional list of players, who joined the game
    pub players: Option<Vec<Player>>,
    /// Optional identifier of the player, who needs to make his / her move next
    pub which_player_turn: Option<String>,
    /// Optional new game state of the game
    pub state: Option<GameState>,
    /// Optional new round number
    ///
    /// Starts by 1 and increments by 1
    pub round_number: Option<usize>,
    /// Optional modified chat instance
    pub chat: Option<Chat>,
    /// Optional mutated card to play in the current round
    pub card_to_play: Option<CardType>,
    /// Optional list of new claims made by users
    pub claims: Option<Vec<Claim>>,
}

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
            round_number: 1,
        }
    }

    /// Creates a new instance of a `Game` struct from a unmutable reference.
    ///
    /// All data is cloned!
    ///
    /// # Example
    ///
    /// ```rust
    ///     let game = Game::new();
    ///     let game_2 = Game::from_ref(&game);
    /// ```
    pub fn from_ref(game: &Game) -> Self {
        Game {
            id: game.id.clone(),
            players: game.players.clone(),
            which_player_turn: game.which_player_turn.clone(),
            state: game.state.clone(),
            started_at: game.started_at.clone(),
            card_to_play: game.card_to_play.clone(),
            chat: game.chat.clone(),
            claims: game.claims.clone(),
            round_number: game.round_number.clone(),
        }
    }

    /// Prepares a Game for it's next round.
    ///
    /// -> Select the first player in the list to start again in the new round
    /// -> Randomly select one card that needs to be played in tht next round
    /// -> Empties the claims list
    /// -> Increments the round counter
    ///
    pub fn prep_for_new_round(&mut self) -> Result<(), ProcessError<Game>> {
        // set select player to the first in the list
        if self.players.len() == 0 {
            return Err(ProcessError::new("Can't prepare the game for the next round! There are no players in the game's list!".to_string(), 
                "ProcessError::new()".to_string(), 
                Some(Game::from_ref(self))));
        }

        self.which_player_turn = self.players[0].id.clone();

        // get new card to play -> with csprng
        self.card_to_play = select_new_card_to_be_played();

        // empty claims list
        self.claims = vec![];
        // increment the round number
        self.round_number += 1;

        Ok(())
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
