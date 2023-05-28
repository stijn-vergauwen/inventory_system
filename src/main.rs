use crate::items::{handle_create_new_item, Item};
use std::fs;
use std::io::{self};

mod items;

// struct Category {
//     name: String,
// }

const PATH_TO_INVENTORY: &str = "data/inventory.json";

fn main() {
    loop {
        println!("Enter a number to select one of the following actions:");

        let choices = vec!["Create item", "Show inventory", "Exit program"];

        match await_user_choice(&choices) {
            1 => create_item(),
            2 => read_items(),
            3 => break,
            _ => continue,
        }
    }
}

fn create_item() {
    let item: Item = handle_create_new_item();

    println!("Created item has following data:");

    println!("{:#?}", item);

    let item_as_json = serde_json::to_string(&item).expect("Item should serialize to JSON");

    fs::write(PATH_TO_INVENTORY, item_as_json.as_bytes()).expect("Item should be written to file");
}

fn read_items() {
    let file_contents =
        fs::read_to_string(PATH_TO_INVENTORY).expect("file contents should be read");

    let item: Item = serde_json::from_str(&file_contents)
        .expect("File contents should be parsed to Item struct");

    println!("{:#?}", item);
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
