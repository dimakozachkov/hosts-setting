use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Host {
    pub is_on: HostStatus,
    pub ip: Ipv4Addr,
    pub title: String,
}

impl Host {
    pub fn new(is_on: HostStatus, ip: Ipv4Addr, title: String) -> Self {
        Self { is_on, ip, title }
    }
}

#[derive(Debug, PartialEq)]
pub enum HostStatus {
    On,
    Off,
}