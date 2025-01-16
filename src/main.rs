use std::io;
struct Item {
    name: String,
    quantity: u16,
    price: f64,
}

enum InventoryActions {}

impl InventoryActions {
    fn add(items: &mut Vec<Item>) {
        // capture new name
        let mut new_name = String::new();
        io::stdin()
            .read_line(&mut new_name)
            .expect("Unable to read line");

        // capture quantity
        let mut new_quanity = String::new();
        let new_quantity_number: u16;

        loop {
            io::stdin()
                .read_line(&mut new_quanity)
                .expect("Unable to read line");

            new_quantity_number = match new_quanity.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please add a valid number");
                    continue;
                }
            };

            break;
        }

        // capture price
        let mut new_price = String::new();
        let new_price_float: f64;

        loop {
            io::stdin()
                .read_line(&mut new_price)
                .expect("Unable to read line");

            new_price_float = match new_price.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                }
            };
            break;
        }

        items.push(Item {
            name: new_name,
            quantity: new_quantity_number,
            price: new_price_float,
        });
    }
    fn remove() {
        println!("remove item");
    }
    fn list() {
        println!("list items");
    }
}
fn main() {
    let mut user_input = String::new();
    let mut items: Vec<Item> = vec![];

    println!("Welcome to my inventory management program");

    loop {
        println!("Select one of the options below: ");
        println!("---------------------------------");
        println!("1. Add an item");
        println!("2. Remove an item");
        println!("3. List items");
        println!("4. Exit");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read line");

        let user_input: u8 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        if user_input == 1 {
            InventoryActions::add(&mut items);
        } else if user_input == 2 {
            InventoryActions::remove();
        } else if user_input == 3 {
            InventoryActions::list();
        } else if user_input == 4 {
            println!("Goodbye");
            break;
        }
    }
}
