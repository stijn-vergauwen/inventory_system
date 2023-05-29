use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::await_user_input;

#[derive(Debug, Serialize, Deserialize)]
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

pub fn create_new_item() -> Item {
    println!("Enter the name of the new item");

    let item_name: String = await_user_input();

    println!("Enter the item's price");

    let item_price: f64 = await_user_input()
        .parse()
        .expect("User's price input should be a number");

    Item::create_new(item_name, item_price)
}

pub fn delete_item_from_list(items: &mut Vec<Item>) {
    println!("Enter the name of the item to delete");

    let index_to_delete = find_item_index_by_name(items, await_user_input());

    match index_to_delete {
        Some(index) => {
            items.remove(index);
        }
        None => println!("Item could not be found!"),
    }
}

pub fn update_amount_of_item(items: &mut Vec<Item>) {
    println!("Enter the name of the item to update");
    let item_index = find_item_index_by_name(items, await_user_input());

    match item_index {
        Some(index) => {
            println!("Enter the new amount for this item");
            let new_amount: u32;
            loop {
                match await_user_input().parse::<u32>() {
                    Ok(number) => {
                        new_amount = number;
                        break;
                    }
                    Err(_) => println!("That's not a valid number, try again"),
                }
            }

            match items.get_mut(index) {
                Some(item) => item.quantity = new_amount,
                None => println!("Item not found!"),
            }
        }
        None => {
            println!("Item name not found!");
        }
    }
}

pub fn update_price_of_item(items: &mut Vec<Item>) {
    println!("Enter the name of the item to update");
    let item_index = find_item_index_by_name(items, await_user_input());

    match item_index {
        Some(index) => {
            println!("Enter the new price for this item");
            let new_price: f64;
            loop {
                match await_user_input().parse::<f64>() {
                    Ok(number) => {
                        new_price = number;
                        break;
                    }
                    Err(_) => println!("That's not a valid price, try again"),
                }
            }

            match items.get_mut(index) {
                Some(item) => item.price = new_price,
                None => println!("Item not found!"),
            }
        }
        None => {
            println!("Item name not found!");
        }
    }
}

fn find_item_index_by_name(items: &mut Vec<Item>, search_name: String) -> Option<usize> {
    let mut item_index: Option<usize> = None;

    for (index, item) in items.iter().enumerate() {
        if item.name == search_name {
            item_index = Some(index);
        }
    }

    item_index
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
