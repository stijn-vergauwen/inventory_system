use std::fs;
use std::io::Error;

use crate::items::Item;

const PATH_TO_INVENTORY: &str = "data/inventory.json";

pub fn save_to_file(items: &Vec<Item>) -> Result<(), Error> {
    let item_as_json = serde_json::to_string(&items)?;
    fs::write(PATH_TO_INVENTORY, item_as_json.as_bytes())?;
    Ok(())
}

pub fn read_from_file() -> Result<Vec<Item>, Error> {
    let file_contents = fs::read_to_string(PATH_TO_INVENTORY)?;
    let item: Vec<Item> = serde_json::from_str(&file_contents)?;
    Ok(item)
}

pub fn add_item_to_file(new_item: Item) -> Result<(), Error> {
    let mut items: Vec<Item> = read_from_file()?;
    items.push(new_item);
    save_to_file(&items)?;
    Ok(())
}
