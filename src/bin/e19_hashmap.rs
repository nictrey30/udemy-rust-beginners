// Topic: HashMap
// * Print the name and number of items in stock for a furniture store
// * Print the total number of items in stock
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;
use std::io;

fn get_num() -> u32 {
    println!("stock: ");
    let mut input = String::new();
    loop {
        input.clear();
        println!("(only positive integers between 1..1000): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.trim().parse() {
            Ok(num) => {
                if num > 1000 {
                    println!("no more than 1000 items at a time");
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("only numbers between 0..1000 allowed!");
                continue;
            }
        };
    }
}

fn get_furniture_type() -> Result<String, String> {
    let mut input = String::new();
    loop {
        input.clear();
        println!("furniture type: ");
        println!("only bed/ chair/ couch/ desk/ table || 'n' to end");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.to_lowercase().trim() {
            "chair" => return Ok("chairs".to_owned()),
            "bed" => return Ok("beds".to_owned()),
            "desk" => return Ok("desks".to_owned()),
            "table" => return Ok("tables".to_owned()),
            "couch" => return Ok("couches".to_owned()),
            "" => {
                println!("empty values not allowed.");
                continue;
            }
            "n" => return Err("stock input finished".to_owned()),
            _ => {
                println!("invalid input");
                continue;
            }
        }
    }
}

fn print_hashmap(hashmap: &HashMap<String, u32>) {
    if hashmap.is_empty() {
        println!("no stock in store!");
    } else {
        for (k, v) in hashmap {
            println!("{:?} {:?}", k, v);
        }
    }
}

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    for key in ["chairs", "beds", "desks", "tables", "couches"] {
        stock.insert(key.to_owned(), 0);
    }
    loop {
        let get_furniture = get_furniture_type();
        match get_furniture {
            Ok(furniture_type) => {
                let new_stock = stock.entry(furniture_type).or_insert(0);
                *new_stock += get_num();
                print_hashmap(&stock);
            }
            Err(e) => {
                println!("{e}");
                println!("Stocks after input: ");
                print_hashmap(&stock);
                break;
            }
        }
    }
}
