use std::sync::{Arc, Mutex};

use crate::{
    network::{NetworkError, NetworkExecutor, NetworkResponse},
    task::Task,
};

pub struct HTTP<S> {
    url: S,
    executor: Arc<Mutex<dyn NetworkExecutor>>,
}

impl<'a, S> Task<'a> for HTTP<S>
where
    S: Send + Sync + Into<String> + 'a,
{
    type Input = ();
    type Output = NetworkResponse;
    type Error = NetworkError;
    fn execute(self: Box<Self>, _input: Self::Input) -> Result<Self::Output, Self::Error> {
        self.executor.lock().unwrap().execute(self.url.into())
    }
}

impl<'a, S> HTTP<S>
where
    S: Send + Sync + Into<String> + 'a,
{
    pub fn new(url: S, executor: Arc<Mutex<dyn NetworkExecutor>>) -> Box<Self> {
        Box::new(HTTP { url, executor })
    }
}
