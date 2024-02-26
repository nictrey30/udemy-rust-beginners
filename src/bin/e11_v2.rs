// Topic: Ownership
// Requirements:
// * Print out the quantity and id number of a grocery item
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
use std::io;

struct Grocery {
    quantity: i32,
    id: i32,
}

fn get_input(input_type: &str) -> i32 {
    let mut buffer = String::new();
    println!("enter {input_type}:");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only integers allowed");
                buffer.clear();
                continue;
            }
        }
    }
}

fn create_grocery() -> Grocery {
    Grocery {
        quantity: get_input("quantity"),
        id: get_input("id"),
    }
}

fn display_grocery_id(grocery: &Grocery) {
    println!("id: {:?}", grocery.id);
}

fn display_grocery_quantity(grocery: &Grocery) {
    println!("quantity: {:?}", grocery.quantity);
}

fn main() {
    let grocery = create_grocery();
    display_grocery_id(&grocery);
    display_grocery_quantity(&grocery);
}
