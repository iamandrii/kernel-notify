use std::sync::{Arc, Mutex};

use kernel_notify::service::factory::NetworkServiceFactory;
use kernel_notify::service::factory::RockyLinux9_4Factory;
use kernel_notify::{network::BlockingNetworkExecutor, service::Service};

fn main() {
    let rocky = RockyLinux9_4Factory::generate_service();
    let executor = Arc::new(Mutex::new(BlockingNetworkExecutor::default()));
    println!("{:?}", rocky.get_latest(executor));
}
