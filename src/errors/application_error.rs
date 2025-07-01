use std::{error, fmt};

/// Global error trait that is implement by custom error types
///
/// It brings all necessary traits that a Rust error struct needs to implement.
///
/// Specific adjustments are made at all indiviual definition.
pub trait ApplicationError: fmt::Display + error::Error + fmt::Debug {}

// impl<T: fmt::Display + error::Error + fmt::Debug> ApplicationError for T {}
