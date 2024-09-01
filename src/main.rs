use std::sync::{Arc, Mutex};

use kernel_notify::notificator::ConsoleNotificator;
use kernel_notify::scheduler;
use kernel_notify::service::factory::NetworkServiceFactory;
use kernel_notify::service::factory::RockyLinux9_4Factory;
use kernel_notify::{network::BlockingNetworkExecutor, service::Service};

fn main() {
    let executor = Arc::new(Mutex::new(BlockingNetworkExecutor::default()));

    let mut scheduler = scheduler::Scheduler::new(
        executor,
        std::time::Duration::from_secs(10),
        Arc::new(Mutex::new(ConsoleNotificator::default())),
    );

    let rocky = RockyLinux9_4Factory::generate_service();
    scheduler.push_service(rocky);

    scheduler.run();
    loop {}
}
