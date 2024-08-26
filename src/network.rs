use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum NetworkError {
    NoInternet,
    GenericReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for NetworkError {
    fn from(value: reqwest::Error) -> Self {
        Self::GenericReqwestError(value)
    }
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self)
    }
}

pub struct NetworkResponse {
    pub code: u16,
    pub body: String,
}

pub type NetworkResult = Result<NetworkResponse, NetworkError>;

pub trait NetworkExecutor: Sync + Send {
    fn execute(&mut self, url: String) -> NetworkResult;
}

#[derive(Default)]
pub struct BlockingNetworkExecutor {}

impl NetworkExecutor for BlockingNetworkExecutor {
    fn execute(&mut self, url: String) -> NetworkResult {
        let resp = reqwest::blocking::get(url)?;
        Ok(NetworkResponse {
            code: resp.status().as_u16(),
            body: resp.text()?,
        })
    }
}
