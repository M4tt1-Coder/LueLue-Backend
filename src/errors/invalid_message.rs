use crate::types::chat::ChatMessage;
use std::fmt;

/// Error for an invalid message created sent by a user.
///
/// # Params
/// - 'message': Describes the occured error
/// - 'origin_message': The message object that caused the error.
pub struct InvalidMessageError {
    /// Describtion of the error
    pub message: String,
    /// Origin of the error
    pub origin_message: ChatMessage,
}

impl fmt::Display for InvalidMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A processed message was invalid! Error: {} & Message object that caused the error: {}",
            self.message, self.origin_message
        )
    }
}

impl fmt::Debug for InvalidMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A processed message was invalid! Error: {} & Message object that caused the error: {}",
            self.message, self.origin_message
        )
    }
}

impl std::error::Error for InvalidMessageError {}
