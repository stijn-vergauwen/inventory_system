use crate::items::{handle_create_new_item, Item};
use std::io;

mod items;

// struct Category {
//     name: String,
// }

fn main() {
    let item: Item = handle_create_new_item();

    println!("Created item has following data:");

    println!("{:?}", item);
}

fn await_user_choice(message: String, choices: &Vec<String>) -> i32 {
    println!("{message}");

    for (index, choice) in choices.iter().enumerate() {
        println!("{}. {}", index + 1, choice);
    }

    let mut selected_number: i32;

    loop {
        selected_number = match await_user_input().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if selected_number > 0 && selected_number < choices.len().try_into().unwrap() {
            return selected_number;
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
