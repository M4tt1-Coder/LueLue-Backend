use serde::{Deserialize, Serialize};

/// Card types for a card game.
///
/// This module defines the different types of cards that can be used in the game.
///
/// Each card type is represented by an enum variant, allowing for easy identification and handling
/// of different card types.
///
/// # Example usage:
/// ```rust
/// use your_crate::card_types::CardType;
/// let card = CardType::King;
/// match card {
///     CardType::King => println!("This is a King card."),
///     CardType::Queen => println!("This is a Queen card."),
///     CardType::Jack => println!("This is a Jack card."),
///     CardType::Ace => println!("This is an Ace card."),
///     CardType::Joker => println!("This is a Joker card."),
///     _ => println!("Unknown card type."),
/// }
/// ```
#[derive(Deserialize, Serialize)]
pub enum CardType {
    /// King card type.
    King,
    /// Queen card type.
    Queen,
    /// Jack card type.
    Jack,
    /// Ace card type.
    Ace,
    /// Joker card type.
    ///
    /// The Joker can be used as a wild card in the game.
    Joker,
}

impl CardType {
    /// Returns a string representation of the card type.
    ///
    /// # Returns
    /// A string slice representing the card type.
    pub fn as_str(&self) -> &str {
        match self {
            CardType::King => "King",
            CardType::Queen => "Queen",
            CardType::Jack => "Jack",
            CardType::Ace => "Ace",
            CardType::Joker => "Joker",
        }
    }
}
