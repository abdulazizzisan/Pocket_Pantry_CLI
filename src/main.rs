mod menu;
mod pantry;

use crate::menu::Command;
use crate::pantry::PantryItem;

fn main() {
    let mut items: Vec<PantryItem> = Vec::new();

    Command::print_banner();
    loop {
        let usr_cmd = match Command::read_menu() {
            Ok(cmd) => cmd,
            Err(_) => continue,
        };

        usr_cmd.execute(&mut items);
    }
}
