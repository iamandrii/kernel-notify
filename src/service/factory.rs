use crate::service::{
    parsers::{Parser, RegexParser},
    NetworkService,
};

pub trait NetworkServiceFactory {
    fn generate_service() -> Box<NetworkService>;
}

pub struct RockyLinux9_4Factory;

impl NetworkServiceFactory for RockyLinux9_4Factory {
    fn generate_service() -> Box<NetworkService> {
        Box::new(NetworkService::new(
            "RockyLinux9_4",
            "https://dl.rockylinux.org/pub/rocky/9.4/BaseOS/x86_64/os/Packages/k/",
            RegexParser::new(r"kernel-core-(5\.14\.0-[0-9\.]+)\.el9_4\.x86_64\.rpm", 1)
                .to_parser_fn(),
        ))
    }
}
