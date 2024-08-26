use std::sync::{Arc, Mutex};

use kernel_notify::error::Error;
use kernel_notify::{
    network::BlockingNetworkExecutor,
    service::{NetworkService, Service},
};
use regex::Regex;

fn main() {
    let rocky = NetworkService::new(
        &String::from("rocky"),
        &String::from("https://dl.rockylinux.org/pub/rocky/9.4/BaseOS/x86_64/os/Packages/k/"),
        |result| match result {
            Err(e) => Err(Error::NetworkError(e)),
            Ok(o) => match o.code {
                200 => {
                    let re =
                        Regex::new(r"kernel-core-(5\.14\.0-[0-9\.]+)\.el9_4\.x86_64\.rpm").unwrap();
                    let caps = re.captures(&o.body);
                    if let Some(caps) = caps {
                        Ok(caps.get(1).map_or("<empty>", |m| m.as_str()).into())
                    } else {
                        Err(Error::GenericError("Unable to parse".into()))
                    }
                }
                _ => Err(Error::GenericError(format!("Unknown code {}", o.code))),
            },
        },
    );
    let executor = Arc::new(Mutex::new(BlockingNetworkExecutor::default()));
    println!("{:?}", rocky.get_latest(executor));
}
