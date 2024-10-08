use crate::network::{NetworkError, NetworkExecutor, NetworkResponse};
use std::sync::{Arc, Mutex};
pub mod factory;
pub mod parsers;

pub type ParserFn = dyn Fn(NetworkResponse) -> Result<String, crate::error::Error> + Send + Sync;

pub trait Service {
    type Input: Send;
    type Output: Send + Eq + Clone;
    type Error: Send;
    fn get_key(&self) -> String;
    fn get_latest(&self, i: Self::Input) -> Result<Self::Output, Self::Error>;
}

pub struct NetworkService {
    key: String,
    url: String,
    parser: Arc<ParserFn>,
}

impl NetworkService {
    pub fn new(key: &str, url: &str, parser: Arc<ParserFn>) -> Self {
        Self {
            key: String::from(key),
            url: String::from(url),
            parser,
        }
    }
}

impl Service for NetworkService {
    type Input = Arc<Mutex<dyn NetworkExecutor>>;

    type Output = String;

    type Error = crate::error::Error;

    fn get_key(&self) -> String {
        self.key.clone()
    }

    fn get_latest(&self, i: Self::Input) -> Result<Self::Output, Self::Error> {
        let resp = i
            .lock()
            .ok()
            .ok_or("Unable to access NetworkExecutor")?
            .execute(self.url.clone());
        match resp {
            Ok(resp) if resp.code == 200 => (self.parser)(resp),
            Ok(resp) => Err(NetworkError::Not200(resp.body).into()),
            Err(resp) => Err(resp.into()),
        }
    }
}
