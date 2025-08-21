use std::{error, fmt};

use serde::Deserialize;

/// Global error trait that is implement by custom error types
///
/// It brings all necessary traits that a Rust error struct needs to implement.
///
/// Specific adjustments are made at all indiviual definition.
pub trait ApplicationError: fmt::Display + error::Error + fmt::Debug {}

/// Error object trait for data types that should be logged in the console or in the error message.
///
/// In some error types the causing object is inbetted in the error message.
pub trait ErrorObject<'a>: Deserialize<'a> + fmt::Display + fmt::Debug {}
