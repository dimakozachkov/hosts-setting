use std::fs;

use crossterm::style::Color;
use ips::{get_disabled_ips, get_enabled_ips};
use terminal_menu::{button, label, list, menu, mut_menu, run, TerminalMenuItem};

mod ips;

const PATH_TO_FILE: &str = "/etc/hosts";

fn main() {
    let ips = ips::get_ips_from_file(PATH_TO_FILE);
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

    menu_items.push(button("exit").colorize(Color::White));

    let menu = menu(menu_items);

    run(&menu);
    let mm = mut_menu(&menu);

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

    fs::write(PATH_TO_FILE, result).expect("Unable to write file");
}
