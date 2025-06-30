// TODO: Implement chat struct functionality

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
///    player_id: Uuid::new_v4(),
///    content: String::from("Hello, world!"),
///    sent_at: Utc::now(),
///    };
/// ```   
pub struct ChatMessage {
    pub player_id: uuid::Uuid,
    pub content: String,
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

// Implementation of 'Chat' struct

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

    // TODO: Add a returning error when the message was invalid or the maximum number was reached

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
    pub fn add_chat_message(&mut self, message: ChatMessage) -> Result<(), ()> {
        // message needs to be long enough
        if message.content.len() <= 0 {
            println!("The message is too short to be added to the chat!");
            return Err(());
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

        return Ok(());
    }
}

// Implementation of 'ChatMessage' struct

impl ChatMessage {
    /// Creates new 'ChatMessage' instance.
    ///
    /// # Returns
    ///
    /// New 'ChatMessage' object with the player id, message body and when it was sent.
    pub fn new(
        player_id: uuid::Uuid,
        content: String,
        sent_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        ChatMessage {
            player_id,
            content,
            sent_at,
        }
    }
}
