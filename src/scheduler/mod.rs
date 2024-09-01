use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    network::NetworkExecutor,
    notificator::Notificator,
    service::{NetworkService, Service},
};

pub struct Scheduler {
    services: Vec<Box<NetworkService>>,
    executor: Arc<Mutex<dyn NetworkExecutor>>,
    time_period: std::time::Duration,
    notificator: Arc<Mutex<dyn Notificator>>,
}

impl Scheduler {
    pub fn new(
        executor: Arc<Mutex<dyn NetworkExecutor>>,
        time_period: std::time::Duration,
        notificator: Arc<Mutex<dyn Notificator>>,
    ) -> Self {
        Self {
            services: Vec::new(),
            executor,
            time_period,
            notificator,
        }
    }

    pub fn push_service(&mut self, service: Box<NetworkService>) {
        self.services.push(service);
    }

    pub fn run(self) {
        thread::spawn(move || loop {
            for service in self.services.iter() {
                let key = service.get_key();
                let value = service.get_latest(self.executor.clone());
                match value {
                    Ok(value) => {
                        if let Ok(notificator) = self.notificator.lock() {
                            notificator.notify(key, value);
                        }
                    }
                    Err(err) => {
                        if let Ok(notificator) = self.notificator.lock() {
                            notificator.notify(key, err.to_string());
                        }
                    }
                }
            }
            thread::sleep(self.time_period);
        });
    }
}
