use crate::menu::Command::*;
use std::io::{Write, stdin, stdout};

pub enum Command {
    Add,
    View,
    Search,
    Update,
    Remove,
    Summary,
    Quit,
    Invalid,
}

impl Command {
    fn get(ordinal: i8) -> Self {
        match ordinal {
            0 => Quit,
            1 => Add,
            2 => View,
            3 => Search,
            4 => Update,
            5 => Remove,
            6 => Summary,
            _ => Invalid,
        }
    }
}

impl Command {
    pub fn print_banner() {
        let banner = "
    ╔═╗┌─┐┌─┐┬┌─┌─┐┌┬┐  ╔═╗┌─┐┌┐┌┌┬┐┬─┐┬ ┬  ╔═╗╦  ╦
    ╠═╝│ ││  ├┴┐├┤  │   ╠═╝├─┤│││ │ ├┬┘└┬┘  ║  ║  ║
    ╩  └─┘└─┘┴ ┴└─┘ ┴   ╩  ┴ ┴┘└┘ ┴ ┴└─ ┴   ╚═╝╩═╝╩";

        print!("{banner}");
    }
    fn print_menu() {
        let menu = "
    1. Add Item
    2. View All Items
    3. Search Item
    4. Update Quantity
    5. Remove Item
    6. View Summary
    0. Quit

    Choose an option: ";

        print!("{menu}");
        stdout().flush().unwrap();
    }
    pub fn read_menu() -> Result<Self, ()> {
        Self::print_menu();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Something went wrong. Please enter again.");
                return Err(());
            }
        }

        let input: i8 = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a valid number.");
                return Err(());
            }
        };
        Ok(Self::get(input))
    }
}
