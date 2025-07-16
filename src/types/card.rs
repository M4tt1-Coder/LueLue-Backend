use std::{clone, fmt};

use serde::{Deserialize, Serialize};

// using statements
use crate::enums::card_types::CardType;

// This module defines the `Card` struct, which represents a card in a card game.

/// The `Card` struct represents a card in a card game.
///
/// It contains a name and a type, which can be one of the predefined card types.
///
/// # Fields
/// - `card_type`: An enum representing the type of the card, such as King, Queen, Jack, Ace, or
#[derive(Deserialize, Serialize)]
pub struct Card {
    /// The type of the card, represented by the `CardType` enum.
    pub card_type: CardType,
}

impl Card {
    /// Creates a new `Card` instance with the specified name and card type.
    ///
    /// # Arguments
    /// - `card_type`: The type of the card, represented by the `CardType` enum.
    ///
    /// # Returns
    /// A new `Card` instance.
    pub fn new(card_type: CardType) -> Self {
        Card { card_type }
    }
}

impl Default for Card {
    /// Provides a default implementation for the `Card` struct.
    ///
    /// # Returns
    /// A new `Card` instance with an empty name and a default card type (King).
    fn default() -> Self {
        Card::new(CardType::King)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card Type: {}", self.card_type)
    }
}

impl clone::Clone for Card {
    fn clone(&self) -> Self {
        Card {
            card_type: match self.card_type {
                CardType::Ace => CardType::Ace,
                CardType::King => CardType::King,
                CardType::Joker => CardType::Jack,
                CardType::Queen => CardType::Queen,
                CardType::Jack => CardType::Jack,
            },
        }
    }
}
