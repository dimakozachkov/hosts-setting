use std::{fs, sync::RwLockWriteGuard};
use crossterm::style::Color;
use host::Host;
use host_service::{get_disabled_hosts, get_enabled_hosts, get_hosts_from_file};
use terminal_menu::{
    button, label, list, menu, mut_menu, run, TerminalMenuItem, TerminalMenuStruct,
};

mod host;
mod host_service;

#[cfg(target_family = "unix")]
const PATH_TO_HOSTS_FILE: &str = "/etc/hosts";
#[cfg(target_family = "windows")]
const PATH_TO_HOSTS_FILE: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

fn main() {
    let hosts = get_hosts_from_file(&PATH_TO_HOSTS_FILE);
    let (enabled_hosts, disabled_hosts) = (get_enabled_hosts(&hosts), get_disabled_hosts(&hosts));

    let mut menu_items: Vec<TerminalMenuItem> = Vec::new();
    menu_items.push(label("Local domains"));
    for host in enabled_hosts.iter() {
        let label = format!("{} {}", host.ip, host.title);
        menu_items.push(list(label, vec!["On", "Off"]).colorize(Color::Green));
    }

    for host in disabled_hosts.iter() {
        let label = format!("{} {}", host.ip, host.title);
        menu_items.push(list(label, vec!["Off", "On"]).colorize(Color::Red));
    }

    menu_items.push(button("SAVE").colorize(Color::White));

    let menu = menu(menu_items);

    run(&menu);
    let mm = mut_menu(&menu);

    let result = generate_hosts_file_content(&hosts, mm);

    save_hosts_file(&PATH_TO_HOSTS_FILE, result);
}

fn generate_hosts_file_content(
    hosts: &Vec<Host>,
    mm: RwLockWriteGuard<TerminalMenuStruct>,
) -> String {
    let mut result = String::new();
    for host in hosts.iter() {
        let label = format!("{} {}", &host.ip.to_string(), &host.title);

        let selected = mm.selection_value(&label);

        if selected == "On" {
            result.push_str(format!("{}\n", &label).as_str());
        } else {
            result.push_str(format!("#{}\n", &label).as_str());
        }
    }

    result
}

fn save_hosts_file(path: &str, content: String) {
    fs::write(path, content).expect("Unable to write file");
}