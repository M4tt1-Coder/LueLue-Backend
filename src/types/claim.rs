// This module defines the `Claim` struct, which represents a claim made by a player in a card
// game.

use serde::{Deserialize, Serialize};

// using statements
use crate::types::card::Card;

// constants

/// Max number of cards that can be claimed in a single claim.
const MAX_CARDS_PER_CLAIM: usize = 4;

/// The `Claim` struct represents a claim made by a player in a card game.
///
/// It contains information about the player who made the claim and the number of cards claimed.
///
/// # Fields
/// - `created_by`: The unique identifier of the player who made the claim.
/// - `number_of_cards`: The number of cards claimed by the player.
#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub created_by: String,
    pub number_of_cards: usize,
    pub cards: Vec<Card>,
}

impl Claim {
    /// Creates a new `Claim` instance with the specified player ID and number of cards.
    ///
    /// # Arguments
    /// - `created_by`: The unique identifier of the player making the claim.
    /// - `number_of_cards`: The number of cards claimed by the player.
    ///
    /// # Returns
    /// A new `Claim` instance.
    pub fn new(created_by: String, number_of_cards: usize, cards: Vec<Card>) -> Self {
        Claim {
            created_by,
            number_of_cards,
            cards,
        }
    }
}
