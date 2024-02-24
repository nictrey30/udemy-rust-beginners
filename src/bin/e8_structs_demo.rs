use std::io;

#[derive(Debug)]
struct GroceryItem {
    stock: i32,
    price: f64,
}

fn get_stock() -> i32 {
    let mut buffer = String::new();
    println!("please input the stock: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only integers please");
                buffer.clear();
                continue;
            }
        }
    }
}

fn get_price() -> f64 {
    let mut buffer = String::new();
    println!("please input a price: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only numbers please");
                buffer.clear();
                continue;
            }
        }
    }
}

fn main() {
    let cereal = GroceryItem {
        stock: get_stock(),
        price: get_price(),
    };
    println!("{:?}", cereal);
}
