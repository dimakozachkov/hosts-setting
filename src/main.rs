use std::{fs, sync::RwLockWriteGuard};
use std::env;

use crossterm::style::Color;
use ips::{get_disabled_ips, get_enabled_ips, get_ips_from_file};
use terminal_menu::{TerminalMenuItem, TerminalMenuStruct, button, label, list, menu, mut_menu, run};

mod ips;

const UNIX_PATH_TO_FILE: &str = "/etc/hosts";
const WINDOWS_PATH_TO_FILE: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

fn main() {
    let path = get_path_to_hosts_file();
    let ips = get_ips_from_file(&path);
    let (enabled_ips, disabled_ips) = (get_enabled_ips(&ips), get_disabled_ips(&ips));

    let mut menu_items: Vec<TerminalMenuItem> = Vec::new();
    menu_items.push(label("Local domains"));
    for ip in enabled_ips.iter() {
        menu_items.push(list(*ip, vec!["On", "Off"]).colorize(Color::Green));
    }

    for ip in disabled_ips.iter() {
        let ip = ip.replace("#", "");
        menu_items.push(list(ip, vec!["Off", "On"]).colorize(Color::Red));
    }

    menu_items.push(button("SAVE").colorize(Color::White));

    let menu = menu(menu_items);

    run(&menu);
    let mm = mut_menu(&menu);

    let result = generate_hosts_file_content(&ips, mm);

    save_hosts_file(&path, result);
}

fn generate_hosts_file_content(ips: &Vec<String>, mm: RwLockWriteGuard<TerminalMenuStruct>) -> String {
    let mut result = "".to_owned();
    
    for ip in ips.iter() {
        let ip = ip.replace("#", "");

        let selected = mm.selection_value(&ip);
        if selected == "On" {
            result.push_str(format!("{}\n", &ip).as_str());
        } else {
            result.push_str(format!("#{}\n", &ip).as_str());
        }
    }

    result
}

fn save_hosts_file(path: &str, content: String) {
    fs::write(path, content).expect("Unable to write file");
}

fn get_path_to_hosts_file() -> String {
    if env::consts::OS == "windows" {
        return WINDOWS_PATH_TO_FILE.to_owned();
    }
    
    UNIX_PATH_TO_FILE.to_owned()
}