use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("The provided value is not an object")]
    NotAnObject,

    #[error("Missing key in object: {0}")]
    MissingKeyInObject(String),

    #[error("Not a string")]
    NotAString,

    #[error("Expected a filled string, but got an empty one")]
    StringNotFilled,

    #[error("Expected a string with a minimum length of {0}, but got a length of {1}")]
    StringNotMinLength(usize, usize),

    #[error("Expected a string with max length of {0}, but a length of {1}")]
    StringExceedsMaxLength(usize, usize),

    #[error("Custom validation error: {0}")]
    Custom(Box<dyn Error>),
}

impl ValidationError {
    pub fn new_custom(error: impl Error + 'static) -> Self {
        ValidationError::Custom(Box::new(error))
    }
}

// impl<T> From<T> for ValidationError
// where
//     T: Error + 'static,
// {
//     fn from(value: T) -> Self {
//         ValidationError::Custom(Box::new(value))
//     }
// }
