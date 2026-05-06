use std::io::{stdin, stdout, Write};

fn main() {
    print_banner();
    let mut user_input = String::new();
    loop {
        print_menu();
        stdout().flush().unwrap();

        match stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(_) => {
                println!("Something went wrong. Please enter again.");
                continue
            }
        }

        let user_input: i8 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if user_input == 0 {
            break;
        }
    }
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
}

fn print_banner() {
    let banner = "
    ╔═╗┌─┐┌─┐┬┌─┌─┐┌┬┐  ╔═╗┌─┐┌┐┌┌┬┐┬─┐┬ ┬  ╔═╗╦  ╦
    ╠═╝│ ││  ├┴┐├┤  │   ╠═╝├─┤│││ │ ├┬┘└┬┘  ║  ║  ║
    ╩  └─┘└─┘┴ ┴└─┘ ┴   ╩  ┴ ┴┘└┘ ┴ ┴└─ ┴   ╚═╝╩═╝╩";

    print!("{banner}");
}
