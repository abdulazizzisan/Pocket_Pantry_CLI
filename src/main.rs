mod menu;
mod pantry;

use crate::menu::Command;

fn main() {
    Command::print_banner();
    loop {
        let usr_cmd = match Command::read_menu() {
            Ok(cmd) => cmd,
            Err(_) => continue,
        };
    }
}

