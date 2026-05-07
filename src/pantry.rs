use crate::pantry::Category::{Fruit, Grain, Other, Protein, Snack, Vegetable};
use crate::pantry::StockStatus::{InStock, LowStock, OutOfStock};

struct PantryItem {
    name: String,
    quantity: u32,
    category: Category,
    status: StockStatus,
}

impl PantryItem {
    fn new(name: String, quantity: u32, category: Category) -> Result<Self, String> {
        let obj = Self {
            name,
            quantity,
            category,
            status: StockStatus::from_quantity(quantity),
        };

        Err("".to_string())
    }
}
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

enum Category {
    Grain,
    Protein,
    Vegetable,
    Fruit,
    Snack,
    Other(String),
}

impl Category {
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

    fn other(desc: String) -> Self{
        Other(desc)
    }
}
