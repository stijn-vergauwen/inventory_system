use items::delete_item_from_list;
use serializer::{add_item_to_file, save_to_file};

use crate::{
    items::{create_new_item, update_amount_of_item, update_price_of_item, Item},
    serializer::read_from_file,
};
use std::io::{self};

mod items;
mod serializer;

// struct Category {
//     name: String,
// }

fn main() {
    loop {
        println!("Enter a number to select one of the following actions:");

        let choices = vec!["Show inventory", "Modify inventory", "Exit program"];

        match await_user_choice(&choices) {
            1 => read_items(),
            2 => handle_crud_action(),
            3 => break,
            _ => continue,
        }
    }
}

fn handle_crud_action() {
    println!("Enter a number to select one of the following actions:");

    let choices = vec![
        "Add item",
        "Remove item",
        "Update item amount",
        "Update item price",
    ];

    match await_user_choice(&choices) {
        1 => create_item(),
        2 => delete_item(),
        3 => {
            let mut items = read_from_file().expect("items should be read from file");
            update_amount_of_item(&mut items);
            match save_to_file(&items) {
                Ok(_) => (),
                Err(error) => println!("Something went wrong. {}", error),
            }
        }
        4 => {
            let mut items = read_from_file().expect("items should be read from file");
            update_price_of_item(&mut items);
            match save_to_file(&items) {
                Ok(_) => (),
                Err(error) => println!("Something went wrong. {}", error),
            }
        }
        _ => (),
    }
}

fn delete_item() {
    let mut items: Vec<Item> = read_from_file().expect("items should be read from file");

    delete_item_from_list(&mut items);

    match save_to_file(&items) {
        Ok(_) => (),
        Err(error) => println!("Something went wrong. {}", error),
    }
}

fn create_item() {
    let created_item: Item = create_new_item();

    match add_item_to_file(created_item) {
        Ok(_) => println!("Item successfully added!"),
        Err(error) => println!("Something went wrong. {}", error),
    }
}

fn read_items() {
    let items: Vec<Item> = read_from_file().expect("items should be read from file");
    println!("{:#?}", items);
}

fn await_user_choice(choices: &Vec<&str>) -> u32 {
    for (index, choice) in choices.iter().enumerate() {
        println!("{}. {}", index + 1, choice);
    }

    let mut selected_number: u32;

    loop {
        selected_number = match await_user_input().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let choice_count: u32 = choices
            .len()
            .try_into()
            .expect("Array length should be valid u32");

        if selected_number > 0 && selected_number <= choice_count {
            return selected_number;
        } else {
            println!("Please choose one of the actions");
        }
    }
}

fn await_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("User input should be valid");

    return user_input.trim().to_string();
}
