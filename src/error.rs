use core::fmt::Display;

use crate::network::NetworkError;

#[derive(Debug)]
pub enum Error {
    NetworkError(crate::network::NetworkError),
    GenericError(String),
    ParsingError(String),
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "NetworkError: {}", e),
            Self::GenericError(e) => write!(f, "GenericError: {}", e),
            Self::ParsingError(e) => write!(f, "ParsingError: {}", e),
        }
    }
}
