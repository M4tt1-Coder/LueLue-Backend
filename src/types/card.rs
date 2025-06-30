// using statements
use crate::enums::card_types::CardType;

// This module defines the `Card` struct, which represents a card in a card game.

/// The `Card` struct represents a card in a card game.
///
/// It contains a name and a type, which can be one of the predefined card types.
///
/// # Fields
/// - `name`: A string representing the name of the card.
/// - `card_type`: An enum representing the type of the card, such as King, Queen, Jack, Ace, or
pub struct Card {
    /// The name of the card.
    pub name: String,
    /// The type of the card, represented by the `CardType` enum.
    pub card_type: CardType,
}

impl Card {
    /// Creates a new `Card` instance with the specified name and card type.
    ///
    /// # Arguments
    /// - `name`: A string representing the name of the card.
    /// - `card_type`: The type of the card, represented by the `CardType` enum.
    ///
    /// # Returns
    /// A new `Card` instance.
    pub fn new(name: String, card_type: CardType) -> Self {
        Card { name, card_type }
    }
}

impl Default for Card {
    /// Provides a default implementation for the `Card` struct.
    ///
    /// # Returns
    /// A new `Card` instance with an empty name and a default card type (King).
    fn default() -> Self {
        Card::new(String::new(), CardType::King)
    }
}
