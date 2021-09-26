use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_ips_from_file(path: &str) -> Vec<String> {
    let reader = get_file(path);
    let mut ips: Vec<String> = Vec::new();

    let re = get_re();

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if let Some(_) = re.captures(&line) {
            ips.push(line.into());
        }
    });

    ips
}

fn get_file(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("File `/etc/hosts` not found!");
    BufReader::new(file)
}

fn get_re() -> Regex {
    Regex::new(r#"#?\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap()
}

pub fn get_enabled_ips(ips: &Vec<String>) -> Vec<&String> {
    ips.iter().filter(|ip| !ip.contains("#")).collect()
}

pub fn get_disabled_ips(ips: &Vec<String>) -> Vec<&String> {
    ips.iter().filter(|ip| ip.contains("#")).collect()
}
