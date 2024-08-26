#[derive(Debug)]
pub enum Error {
    NetworkError(crate::network::NetworkError),
    GenericError(String),
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
