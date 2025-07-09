use axum::Json;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::errors::{
    application_error::ErrorObject, bad_client_request::BadClientRequest,
    invalid_message::InvalidMessageError,
};

// constants

/// The maximum number of messages that can be stored in a chat.
const MAX_CHAT_MESSAGE_LENGTH: usize = 50;

/// Represents a chat in the game, containing messages exchanged between players.
///
/// The `Chat` struct holds a vector of `ChatMessage` instances and tracks the number of messages.
///
/// # Example usage:
/// ```rust
/// use uuid::Uuid;
/// use chrono::Utc;
/// use your_crate::chat::{Chat, ChatMessage};
/// let mut chat = Chat {
///    messages: Vec::new(),
///    number_of_messages: 0,
///
///    };
/// ```
#[derive(Deserialize, Serialize)]
pub struct Chat {
    pub messages: Vec<ChatMessage>,
    pub number_of_messages: usize,
}

/// Represents a chat message in the game, containing the player ID and the message content.
///
/// The `ChatMessage` struct holds the ID of the player who sent the message, the content of the
/// message,
/// and the timestamp of when the message was sent.
///
/// # Example usage:
/// ```rust
/// use uuid::Uuid;
/// use chrono::Utc;
/// use your_crate::chat::ChatMessage;
/// let message = ChatMessage {
///    player_id: Uuid::new_v4().to_string(),
///    content: String::from("Hello, world!"),
///    sent_at: Utc::now().to_string(),
///    };
/// ```  
#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub player_id: String,
    pub content: String,
    pub sent_at: String, // as chrono::DateTime<chrono::Utc>,
}

// Implementation of 'Chat' struct

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}

impl Chat {
    /// Creates a fresh 'Chat' instance.
    ///
    /// # Returns
    /// - 'Chat' object with no messages
    pub fn new() -> Self {
        Chat {
            messages: vec![],
            number_of_messages: 0,
        }
    }

    /// Resets the 'Chat' instance.
    ///
    /// Messages will be deleted and number of messages set to null.
    pub fn reset(&mut self) {
        self.number_of_messages = 0;
        self.messages = vec![];
    }

    /// Tries adding a message to the 'Chat' instance of a game.
    ///
    /// When the message vector is full then the oldest massage is deleted and the new message was
    /// added.
    ///
    /// # Errors
    ///
    /// When the message itself is too short or has no content then it isn't stored or saved
    ///
    /// # Returns
    ///
    /// Result<(), ApplicationError> - When the message was invalid.
    pub fn add_chat_message(&mut self, message: ChatMessage) -> Result<(), InvalidMessageError> {
        // message needs to be long enough
        if message.content.is_empty() {
            println!("The message is too short to be added to the chat!");
            return Err(InvalidMessageError {
                message: "Too short message content! Must not be of length 0!".to_string(),
                origin_message: message,
            });
        }

        // check if the maximum number of messages was reached
        if self.number_of_messages >= MAX_CHAT_MESSAGE_LENGTH {
            println!("Maximum number of chat messages has been reached! Deleting oldest message to add the new one.");
            self.messages.remove(0);
            self.messages.push(message);
            return Ok(());
        }

        // add the message to the chat in the normal case
        self.number_of_messages += 1;
        self.messages.push(message);

        Ok(())
    }
}

// Implementation of 'ChatMessage' struct

impl ChatMessage {
    /// Creates new 'ChatMessage' instance.
    ///
    /// # Returns
    ///
    /// New 'ChatMessage' object with the player id, message body and when it was sent.
    ///
    /// # Errors
    ///
    /// Returns 'BadClientRequest' if the a client provided invalid data.
    pub fn new(
        player_id: String,
        content: String,
        sent_at: String,
    ) -> Result<Self, BadClientRequest<ChatMessage>> {
        if content.is_empty() || player_id.is_empty() || sent_at.is_empty() {
            return Err::<ChatMessage, BadClientRequest<_>>(BadClientRequest {
                bad_data: Json(ChatMessage {
                    player_id: player_id.clone(),
                    sent_at: sent_at.clone(),
                    content: content.clone(),
                }),
                message: format!(
                    "The provided data by player with id: {} for a chat message was not valid!",
                    &player_id
                ),
            });
        };
        Ok(ChatMessage {
            player_id,
            content,
            sent_at,
        })
    }
}
impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[
           PlayerID: {},
           Content: {},
           Sent at: {}
            ]",
            self.player_id, self.content, self.sent_at
        )
    }
}

impl fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[
           PlayerID: {},
           Content: {},
           Sent at: {}
            ]",
            self.player_id, self.content, self.sent_at
        )
    }
}

impl ErrorObject<'_> for ChatMessage {}
