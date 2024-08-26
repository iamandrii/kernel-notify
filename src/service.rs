use crate::network::{NetworkExecutor, NetworkResult};
use std::sync::{Arc, Mutex};

pub trait Service {
    type Input: Send;
    type Output: Send + Eq + Clone;
    type Error: Send;
    fn get_key(&self) -> String;
    fn get_latest(&self, i: Self::Input) -> Result<Self::Output, Self::Error>;
}

pub struct NetworkService<F: Fn(NetworkResult) -> Result<String, crate::error::Error>> {
    key: String,
    url: String,
    parser: F,
}

impl<F: Fn(NetworkResult) -> Result<String, crate::error::Error>> NetworkService<F> {
    pub fn new(key: &String, url: &String, parser: F) -> Self {
        Self {
            key: key.clone(),
            url: url.clone(),
            parser,
        }
    }
}

impl<F: Fn(NetworkResult) -> Result<String, crate::error::Error>> Service for NetworkService<F> {
    type Input = Arc<Mutex<dyn NetworkExecutor>>;

    type Output = String;

    type Error = crate::error::Error;

    fn get_key(&self) -> String {
        self.key.clone()
    }

    fn get_latest(&self, i: Self::Input) -> Result<Self::Output, Self::Error> {
        (self.parser)(
            i.lock()
                .ok()
                .ok_or("Unable to access NetworkExecutor")?
                .execute(self.url.clone()),
        )
    }
}
