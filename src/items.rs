pub mod items {
    use crate::Category;

    pub struct Item {
        name: String,
        quantity: u32,
        price: f64,
        category: Category,
        sku: u32,
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
                category: Category {
                    name: String::from("default"),
                },
                sku: 20,
            };

            assert_eq!(item.name, "test item");
        }
    }
}
