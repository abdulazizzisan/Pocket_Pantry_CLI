use crate::menu::Command::*;
use crate::pantry::PantryItem;
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
    pub fn execute(&self, item_vector: &mut Vec<PantryItem>) {
        match self {
            Add => {
                let mut item = PantryItem::from_user();
                for itm in item_vector.iter() {
                    if itm.name == item.name {
                        println!(
                            "An item already exists with same name. Incrementing the quantity."
                        );
                        item.set_quantity(item.get_quantity());
                        println!("Item updated successfully.");
                        return;
                    }
                }
                item_vector.push(item);
                println!("Item added successfully.");
            }
            View => {
            }
            Search => {}
            Update => {}
            Remove => {}
            Summary => {}
            Quit => {}
            Invalid => {}
        }
    }
}

impl Command {
    fn from_ordinal(i: i8) -> Self {
        match i {
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
        Ok(Self::from_ordinal(input))
    }
}
