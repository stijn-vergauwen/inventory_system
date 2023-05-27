// use crate::Category;
use rand::Rng;

use crate::await_user_input;

#[derive(Debug)]
pub struct Item {
    name: String,
    quantity: u32,
    price: f64,
    sku: u32,
}

impl Item {
    pub fn create_new(name: String, price: f64) -> Item {
        let mut rng = rand::thread_rng();

        Item {
            name,
            price,
            quantity: 0,
            sku: rng.gen_range(10_000..99_999),
        }
    }
}

pub fn handle_create_new_item() -> Item {
    println!("Enter the name of the new item");

    let item_name: String = await_user_input();

    println!("Enter the item's price");

    let item_price: f64 = await_user_input()
        .parse()
        .expect("User's price input should be a number");

    return Item::create_new(item_name, item_price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_item() {
        let item = Item {
            name: String::from("test item"),
            quantity: 4,
            price: 100.0,
            sku: 20,
        };

        assert_eq!(item.name, "test item");
        assert_eq!(item.price, 100.0);
    }

    #[test]
    fn has_5_digit_sku() {
        let item = Item::create_new(String::from("test item"), 80.0);

        assert!(item.sku > 9000 && item.sku < 100_000);
    }
}
