// Topic: HashMap
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * ex: The store has: 5 Chairs, 3 Beds, 2 Tables, 0 Couches
// * Print the total number of items in stock
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Furniture {
    Chair,
    Bed,
    Couch,
    Cupboard,
    Table,
}

#[derive(Debug)]
struct FurnitureShop {
    furniture_type: Furniture,
    stock: i32,
}

impl FurnitureShop {
    fn new() -> Self {
        return Self {
            furniture_type: get_furniture(),
            stock: get_int(),
        };
    }
}

fn print_furniture(furniture: &FurnitureShop) {
    if furniture.stock == 0 {
        println!("{:?} out of stock", furniture.furniture_type);
    } else {
        println!("{} {:?}", furniture.stock, furniture.furniture_type)
    }
}

fn get_furniture() -> Furniture {
    let mut buffer = String::new();
    println!("type of furniture: chair/table/bed/couch/cupboard");
    loop {
        buffer.clear();
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error");
        }
        match buffer.to_lowercase().trim() {
            "chair" => return Furniture::Chair,
            "bed" => return Furniture::Bed,
            "couch" => return Furniture::Couch,
            "table" => return Furniture::Table,
            "cupboard" => return Furniture::Cupboard,
            _ => {
                println!("chair/table/bed/couch/cupboard only");
                continue;
            }
        }
    }
}

fn get_int() -> i32 {
    let mut buffer = String::new();
    println!("stock: ");
    loop {
        buffer.clear();
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error");
        }
        match buffer.to_lowercase().trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only integers allowed");
                continue;
            }
        }
    }
}

fn main() {
    let mut store = HashMap::new();
    store.insert(1, FurnitureShop::new());
    store.insert(2, FurnitureShop::new());
    store.insert(3, FurnitureShop::new());
    store.insert(4, FurnitureShop::new());
    for furniture in store.values() {
        print_furniture(&furniture);
    }
}
