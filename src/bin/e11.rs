// Topic: Ownership
// Requirements:
// * Print out the quantity and id number of a grocery item

// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

use std::io;

#[derive(Debug, PartialEq)]
enum GroceriesAttr {
    Quantity,
    Id,
}

#[derive(Debug)]
struct Groceries {
    grocery_type: String,
    id_no: u32,
    quantity: u32,
}

impl Groceries {
    fn display_qty(&self) -> u32 {
        self.quantity
    }
    fn display_id(&self) -> u32 {
        self.id_no
    }
}

fn display_info(grocery: &Groceries) {
    println!(
        "{:?} with id: {:?}, qty: {:?}",
        grocery.grocery_type.trim(),
        grocery.display_id(),
        grocery.display_qty()
    );
}

fn get_num(num_type: GroceriesAttr) -> u32 {
    let mut value = String::new();
    let msg: String = if num_type == GroceriesAttr::Id {
        "id".to_owned()
    } else {
        "qty".to_owned()
    };
    loop {
        value.clear();
        println!("input {msg}: ");
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only positive numbers allowed");
                continue;
            }
        };
        return value;
    }
}

fn get_name() -> String {
    let mut value = String::new();
    println!("grocery name: ");
    io::stdin()
        .read_line(&mut value)
        .expect("failed to read line");
    value
}

fn main() {
    let my_grocery = Groceries {
        grocery_type: get_name(),
        id_no: get_num(GroceriesAttr::Id),
        quantity: get_num(GroceriesAttr::Quantity),
    };
    display_info(&my_grocery);
}
