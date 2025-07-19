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

    /// The last time a player requested a status updated.
    ///
    /// If the time exceeds 5 minutes the player will be deleted from the gaming session.
    pub last_time_updated: String,
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
            last_time_updated: chrono::Utc::now().to_string(),
        }
    }
}
