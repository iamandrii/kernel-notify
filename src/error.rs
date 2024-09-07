use core::fmt::Display;

use crate::{network::NetworkError, task::factory::SelectorError};

#[derive(Debug)]
pub enum Error {
    NetworkError(crate::network::NetworkError),
    GenericError(String),
    ParsingError(String),
    SelectorError(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::GenericError(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::GenericError(String::from(value))
    }
}

impl From<NetworkError> for Error {
    fn from(value: NetworkError) -> Self {
        Self::NetworkError(value)
    }
}

impl From<SelectorError> for Error {
    fn from(value: SelectorError) -> Self {
        match value {
            SelectorError::Empty => Self::SelectorError("Empty set".to_string()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "NetworkError: {}", e),
            Self::GenericError(e) => write!(f, "GenericError: {}", e),
            Self::ParsingError(e) => write!(f, "ParsingError: {}", e),
            Self::SelectorError(e) => write!(f, "SelectorError: {}", e),
        }
    }
}
