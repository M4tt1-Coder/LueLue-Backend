use serde::{Deserialize, Serialize};

use crate::types::card::Card;

/// Player struct representing a player in the game system.
///
/// He / she can be identified by a unique ID.
///
/// Contains data set by the user like the name, etc. ...
#[derive(Deserialize, Serialize)]
pub struct Player {
    /// Unique identifier of the player.
    pub id: String,

    /// Name of the player.
    pub name: String,

    /// Score of the player in the game.
    pub score: usize,

    /// The date and time when the player joined the game.
    pub joined_at: String,

    /// The cards assigned to the player.
    pub assigned_cards: Vec<Card>,
}

impl Player {
    /// Creates a new `Player` instance with the specified name and an empty card list.
    ///
    /// # Arguments
    /// - `name`: A string representing the name of the player.
    ///
    /// # Returns
    /// A new `Player` instance with a unique ID, the provided name, and an empty card list.
    pub fn new(name: String) -> Self {
        Player {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            score: 0,
            joined_at: chrono::Utc::now().to_string(),
            assigned_cards: Vec::new(),
        }
    }
}
