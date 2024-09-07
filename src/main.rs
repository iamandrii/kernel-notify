use std::sync::{Arc, Mutex};

use kernel_notify::error::Error;
use kernel_notify::network::BlockingNetworkExecutor;
use kernel_notify::network::NetworkError;
use kernel_notify::network::NetworkResponse;
use kernel_notify::notificator::ConsoleNotificator;
use kernel_notify::scheduler;
use kernel_notify::service::factory::NetworkServiceFactory;
use kernel_notify::service::factory::RockyLinux9_4Factory;
use kernel_notify::task;
use kernel_notify::task::factory;

fn main() {
    let task_builder: task::TaskBuilder<'_, (), (), _> = task::TaskBuilder::new()
        .unsafe_chain(factory::HTTP::new(
            "https://dl.rockylinux.org/pub/rocky/9.4/BaseOS/x86_64/os/Packages/k/",
            Arc::new(Mutex::new(BlockingNetworkExecutor {})),
        ))
        .chain::<_, _, kernel_notify::error::Error>(factory::Lambda::new(
            |response: NetworkResponse| match response.code {
                200 => Ok(response.body),
                _ => Err(NetworkError::Not200(format!(
                    "Unable to fetch kernel-core RPMs: {}",
                    response.code
                ))),
            },
        ))
        .safe_chain(factory::Regex::new(
            r"kernel-core-(5\.14\.0-[0-9\.]+)\.el9_4\.x86_64\.rpm",
            1,
        ))
        .chain::<_, _, kernel_notify::error::Error>(factory::First::new())
        .safe_chain(factory::Console::new());

    let task = task_builder.build();

    task.execute(()).unwrap();
}
