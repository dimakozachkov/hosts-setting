use std::{fs, sync::RwLockWriteGuard, net::Ipv4Addr, process};
use crossterm::style::Color;
use host::{Host, HostStatus};
use host_service::{get_hosts_by_status, get_hosts_from_file};
use terminal_menu::{
    button, label, list, menu, mut_menu, run, TerminalMenuItem, TerminalMenuStruct,
};
use is_root::is_root;

mod host;
mod host_service;

#[cfg(target_family = "unix")]
const PATH_TO_HOSTS_FILE: &str = "/etc/hosts";
#[cfg(target_family = "windows")]
const PATH_TO_HOSTS_FILE: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

const MENU_ITEM_POINT_ON: &str = "On";
const MENU_ITEM_POINT_OFF: &str = "Off";

fn main() {
    if !is_root() {
        println!("To use the program you must run the program with root/administrator privileges!");
        process::exit(0);
    }

    let hosts = get_hosts_from_file(&PATH_TO_HOSTS_FILE);
    let enabled_hosts = get_hosts_by_status(&hosts, HostStatus::On);
    let disabled_hosts = get_hosts_by_status(&hosts, HostStatus::Off);

    let mut menu_items: Vec<TerminalMenuItem> = Vec::new();
    menu_items.push(label("Local domains"));

    for host in enabled_hosts.iter() {
        menu_items.push(get_menu_item(host.ip, &host.title, vec![MENU_ITEM_POINT_ON, MENU_ITEM_POINT_OFF], Color::Green));
    }

    for host in disabled_hosts.iter() {
        menu_items.push(get_menu_item(host.ip, &host.title, vec![MENU_ITEM_POINT_OFF, MENU_ITEM_POINT_ON], Color::Red));
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

        if selected == MENU_ITEM_POINT_ON {
            result.push_str(format!("{}\n", &label).as_str());
        } else {
            result.push_str(format!("#{}\n", &label).as_str());
        }
    }

    result
}

fn get_menu_item(ip: Ipv4Addr, title: &str, points: Vec<&str>, color: Color) -> TerminalMenuItem {
    let label = format!("{} {}", ip, title);
    list(label, points).colorize(color)
}

fn save_hosts_file(path: &str, content: String) {
    fs::write(path, content).expect("Unable to write file");
}