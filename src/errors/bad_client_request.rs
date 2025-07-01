use std::fmt::{self, Debug, Display};

use axum::{http::StatusCode, Json};
use serde::Deserialize;

/// Error type for all request with invalid data a client sends to the backend.
///
/// # Props
///
/// - 'message': Description what error occured.
/// - 'bad_data': The data object of type 'T' that caused the error.
pub struct BadClientRequest<'a, T: Deserialize<'a> + Display + Debug> {
    /// Message for the client what he / she did wrong
    pub message: String,
    /// Data object as origin for the error
    pub bad_data: Json<&'a T>,
}

impl<'a, T: Display + Deserialize<'a> + Debug> Display for BadClientRequest<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bad request was sent by a client! Error: {} ... caused by: {:?}!",
            self.message, self.bad_data
        )
    }
}

impl<'a, T: Display + Debug + Deserialize<'a>> Debug for BadClientRequest<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bad request was sent by a client! Error: {} ... caused by: {:?}!",
            self.message, self.bad_data
        )
    }
}

impl<'a, T: Display + Debug + Deserialize<'a>> std::error::Error for BadClientRequest<'a, T> {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<'a, T: Debug + Display + Deserialize<'a>> BadClientRequest<'a, T> {
    /// Resembling http status code for a bad request
    pub const STATUS_CODE: StatusCode = StatusCode::BAD_REQUEST;

    /// Creates a new 'BadClientRequest' instance with a indiviual error message.
    ///
    /// # Params
    ///
    /// - 'message': Error message
    /// - 'bad_data': Request body or wrongly created data object
    ///
    /// # Returns
    ///
    /// BadClientRequest<'a, T>: Specific error for different endpoints for example.
    ///
    pub fn new(message: String, bad_data: Json<&'a T>) -> BadClientRequest<'a, T> {
        BadClientRequest { message, bad_data }
    }
}
