use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::Ipv4Addr;
use std::ops::Index;

use crate::host::{Host, HostStatus};

pub fn get_hosts_from_file(path: &str) -> Vec<Host> {
    let reader = get_file(path);
    let mut hosts: Vec<Host> = Vec::new();

    let re = get_re();

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if let Some(cap) = re.captures(&line) {
            let is_on = if cap.index(1).contains("#") {
                HostStatus::Off
            } else {
                HostStatus::On
            };
            let ip: Ipv4Addr = cap.index(2).parse().unwrap();
            hosts.push(Host::new(is_on, ip, cap.index(3).to_string()));
        }
    });

    hosts
}

fn get_file(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("File `/etc/hosts` not found!");
    BufReader::new(file)
}

fn get_re() -> Regex {
    Regex::new(r#"^(#?)(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\s+([\w\.\-_]+)"#).unwrap()
}

pub fn get_hosts_by_status(hosts: &Vec<Host>, status: HostStatus) -> Vec<&Host> {
    hosts
        .iter()
        .filter(|host| host.is_on == status)
        .collect()
}
