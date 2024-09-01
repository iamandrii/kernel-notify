use std::sync::{Arc, Mutex};

use kernel_notify::service::parsers::Parser;
use kernel_notify::service::parsers::RegexParser;
use kernel_notify::{
    network::BlockingNetworkExecutor,
    service::{NetworkService, Service},
};

fn main() {
    let rocky = NetworkService::new(
        &String::from("rocky"),
        &String::from("https://dl.rockylinux.org/pub/rocky/9.4/BaseOS/x86_64/os/Packages/k/"),
        RegexParser::new(r"kernel-core-(5\.14\.0-[0-9\.]+)\.el9_4\.x86_64\.rpm", 1).to_parser_fn(),
    );
    let executor = Arc::new(Mutex::new(BlockingNetworkExecutor::default()));
    println!("{:?}", rocky.get_latest(executor));
}
