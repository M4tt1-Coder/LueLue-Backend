// This module defines the `Claim` struct, which represents a claim made by a player in a card
// game.

use std::fmt;

use axum::Json;
use serde::{Deserialize, Serialize};

// using statements
use crate::{
    errors::{application_error::ErrorObject, bad_client_request::BadClientRequest},
    types::card::Card,
};

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
#[derive(Deserialize, Serialize, Clone)]
pub struct Claim {
    /// Id of the user that placed the claim on the stack
    pub created_by: String,
    /// Number of cards used in the claim
    pub number_of_cards: usize,
    /// List of placed cards in the claim
    pub cards: Vec<Card>,
}

impl Claim {
    /// Creates a new `Claim` instance with the specified player ID and number of cards.
    ///
    /// # Arguments
    /// - `created_by`: The unique identifier of the player making the claim.
    /// - `number_of_cards`: The number of cards claimed by the player.
    /// - 'cards' : List of cards with a maximum number of 4
    ///
    /// # Error
    ///
    /// Return a 'BadClientRequest<Claim>' error when the provided error of the user is invalid.
    ///
    /// # Returns
    /// A new `Claim` instance.
    pub fn new(
        created_by: String,
        number_of_cards: usize,
        cards: Vec<Card>,
    ) -> Result<Self, BadClientRequest<Claim>> {
        if number_of_cards > MAX_CARDS_PER_CLAIM {
            return Err::<Claim, BadClientRequest<Claim>>(BadClientRequest {
                message: "The user handed in an invalid claim object!".to_string(),
                bad_data: Json(Claim {
                    created_by: created_by.clone(),
                    number_of_cards,
                    cards: cards.clone(),
                }),
            });
        };
        Ok(Claim {
            created_by,
            number_of_cards,
            cards,
        })
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
        Created By: {},
        Number of Cards: {},
        All cards: {:?}
            ",
            self.created_by, self.number_of_cards, self.cards
        )
    }
}

impl fmt::Debug for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
        Created By: {},
        Number of Cards: {},
        All cards: {:?}
            ",
            self.created_by, self.number_of_cards, self.cards
        )
    }
}

impl<'a> ErrorObject<'a> for Claim {}
