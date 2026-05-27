use std::io::{stdin};

use crate::pantry::Category::{Fruit, Grain, Other, Protein, Snack, Vegetable};
use crate::pantry::StockStatus::{InStock, LowStock, OutOfStock};

#[derive(Debug)]
pub struct PantryItem {
    pub name: String,
    quantity: u32,
    category: Category,
    status: StockStatus,
}

impl PantryItem {
    fn new(name: String, quantity: u32, category: Category) -> Self {
        let obj = Self {
            name,
            quantity,
            category,
            status: StockStatus::from_quantity(quantity),
        };

        obj
    }

    pub fn from_user() -> Self {
        let mut name = String::new();
        let mut quantity = String::new();
        let mut category = String::new();

        // take name input
        println!("Enter item name: ");
        stdin().read_line(&mut name).expect("Error reading name.");
        let name = name.trim().to_string();

        // take quantity input
        println!("Enter quantity(Number only):");
        stdin()
            .read_line(&mut quantity)
            .expect("Error reading quantity.");
        let quantity: u32 = match quantity.trim().parse() {
            Ok(val) => val,
            Err(_) => 0,
        };

        // take category input
        println!("Select a category:");
        println!("{}", Category::string_label());
        stdin()
            .read_line(&mut category)
            .expect("Error reading category");
        let category = category
            .trim()
            .parse::<i8>()
            .expect("please enter valid number.");
        let category: Category = match category {
            0..=4 => Category::from_ordinal(category).unwrap(),
            5 => {
                println!("Please enter short desc of the category:");
                let mut desc = String::new();
                stdin().read_line(&mut desc).expect("IO err while reading ");
                Category::other(desc.trim().to_string())
            }
            _ => panic!("Invalid index for category."),
        };

        Self::new(name, quantity, category)
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = quantity + self.quantity;
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
}


#[derive(Debug)]
enum StockStatus {
    InStock,
    LowStock,
    OutOfStock,
}

impl StockStatus {
    fn from_quantity(quantity: u32) -> Self {
        match quantity {
            q if q == 0 => OutOfStock,
            q if q < 30 => LowStock,
            _ => InStock,
        }
    }
}

#[derive(Debug)]
enum Category {
    Grain,
    Protein,
    Vegetable,
    Fruit,
    Snack,
    Other(String),
}

impl Category {
    fn string_label() -> String {
        "
            0. Grain,
            1. Protein,
            2. Vegetable,
            3. Fruit,
            4. Snack,
            5. Other
            "
        .to_string()
    }
    fn from_ordinal(i: i8) -> Result<Self, String> {
        match i {
            0 => Ok(Grain),
            1 => Ok(Protein),
            2 => Ok(Vegetable),
            3 => Ok(Fruit),
            4 => Ok(Snack),
            _ => Err("Invalid index id.".to_string()),
        }
    }

    fn other(desc: String) -> Self {
        Other(desc)
    }
}
