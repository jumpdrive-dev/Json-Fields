use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("The provided value is not an object")]
    NotAnObject,

    #[error("Missing key in object: {0}")]
    MissingKeyInObject(String),

    #[error("Custom validation error: {0}")]
    Custom(Box<dyn Error>),
}

impl<T> From<T> for ValidationError
where T: Error + 'static
{
    fn from(value: T) -> Self {
        ValidationError::Custom(Box::new(value))
    }
}
