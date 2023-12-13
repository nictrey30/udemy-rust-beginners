// Project 1: Interactive bill manager
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it throughout your program.
// * Create your program starting at level 1. Once finished, advance to the next level.
#![allow(dead_code, unused_variables, unused_imports)]
use rand::Rng;
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Bill {
    name: Option<String>,
    amount: f64,
}

impl Bill {
    fn get_name() -> Option<String> {
        let name = get_input(Some("name"));
        loop {
            match name {
                Ok(input) => match input.trim() {
                    "" => return None,
                    _ => return Some(input),
                },
                Err(e) => {
                    println!("error: {:?}", e);
                    return None;
                }
            }
        }
    }

    fn get_amount() -> f64 {
        let mut amount = String::new();
        loop {
            amount.clear();
            amount = match get_input(Some("amount")) {
                Ok(string) => string,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            };
            match amount.parse::<f64>() {
                Ok(num) => return num,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            };
        }
    }

    fn create_bill(name: Option<String>, amount: f64) -> Self {
        Self { name, amount }
    }
}

fn generate_id() -> u32 {
    rand::thread_rng().gen_range(1..1000)
}

fn get_id() -> u32 {
    loop {
        match get_input(Some("id")) {
            Ok(inner_string) => match inner_string.parse() {
                Ok(inner_num) => return inner_num,
                Err(_) => {
                    println!("only u32 numbers allowed");
                    continue;
                }
            },
            Err(e) => {
                println!("error: {:?}", e);
                continue;
            }
        }
    }
}

fn get_input(input: Option<&str>) -> io::Result<String> {
    match input {
        // buf = name/amount to print bill name/ bill amount, None to just get the input without printing something first
        Some(buf) => println!("bill {}", buf),
        None => (),
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase())
}

fn continue_input(option: &str) -> bool {
    let mut answer = String::new();
    loop {
        answer.clear();
        println!("{} y/n", option);
        let answer = match get_input(None) {
            Ok(string) => string,
            Err(e) => {
                println!("error getting y/n: {:?}", e);
                continue;
            }
        };
        match answer.trim() {
            "y" => return true,
            "n" => return false,
            _ => {
                println!("only y/n");
                continue;
            }
        };
    }
}

fn print_hashmap(hashmap: &HashMap<u32, Bill>) {
    for (key, value) in hashmap.iter() {
        println!("id: {} -> value: {:?}", key, value);
    }
}

fn delete_bills(hashmap: &mut HashMap<u32, Bill>) {
    let mut delete_option = continue_input("delete item");
    let mut current_total_elements = hashmap.len();
    while current_total_elements > 0 && delete_option == true {
        println!("{} elements remaining", current_total_elements);
        'outer: loop {
            let delete_id = get_id();
            for key in hashmap.keys() {
                if key == &delete_id {
                    hashmap.remove(&delete_id);
                    print_hashmap(hashmap);
                    break 'outer;
                }
            }
            println!("id not valid.please input ids only from the bills list.");
        }
        current_total_elements = hashmap.len();
        if current_total_elements > 0 {
            delete_option = continue_input("delete item");
            if !delete_option {
                print_hashmap(hashmap);
            }
        } else {
            println!("bill list empty!");
            return ();
        }
    }
}

fn clone_hashmap(hashmap: &HashMap<u32, Bill>) -> HashMap<u32, Bill> {
    let mut edited_hashmap: HashMap<u32, Bill> = HashMap::new();
    for (k, v) in hashmap {
        let new_bill: Bill;
        let new_name: Option<String> = match &v.name {
            Some(inner_name) => Some(inner_name.to_owned()),
            None => None,
        };
        new_bill = Bill::create_bill(new_name, v.amount);
        edited_hashmap.insert(*k, new_bill);
    }
    edited_hashmap
}

fn edit_bills(hashmap: &HashMap<u32, Bill>) -> HashMap<u32, Bill> {
    let mut edited_hashmap: HashMap<u32, Bill> = clone_hashmap(hashmap);
    print_hashmap(&edited_hashmap);
    loop {
        match continue_input("edit item") {
            true => 'inner: loop {
                let edit_id = get_id();
                let mut found = false;
                for key in hashmap.keys() {
                    if key == &edit_id {
                        found = true;
                        edited_hashmap = edit_bill(edit_id, &edited_hashmap);
                        print_hashmap(&edited_hashmap);
                        match continue_input("edit item") {
                            true => continue,
                            false => return edited_hashmap,
                        }
                    }
                }
                if !found {
                    println!("incorrect id, please input only valid id's");
                    continue 'inner;
                }
            },
            false => {
                print_hashmap(&edited_hashmap);
                return edited_hashmap;
            }
        }
    }
}

fn edit_bill(id: u32, hashmap: &HashMap<u32, Bill>) -> HashMap<u32, Bill> {
    let current_bill = match hashmap.get(&id) {
        Some(bill) => Bill {
            name: match &bill.name {
                Some(inner_name) => Some(inner_name.to_owned()),
                None => None,
            },
            amount: bill.amount,
        },
        None => Bill {
            name: None,
            amount: 0.00,
        },
    };
    let mut edited_hashmap: HashMap<u32, Bill> = clone_hashmap(hashmap);
    println!("current bills:");
    let value = edited_hashmap.entry(id).or_insert(Bill {
        name: None,
        amount: 0.00,
    });
    let new_name: Option<String>;
    let new_amout: f64;
    loop {
        println!("edit name/amount of {:?}", current_bill);
        println!("input-> name or amount:");
        let choice = get_input(None);
        match choice {
            Ok(string) => match string.trim() {
                "name" => {
                    new_name = Bill::get_name();
                    match new_name {
                        Some(inner_name) => {
                            *value = Bill::create_bill(Some(inner_name), value.amount);
                        }
                        None => {
                            *value = Bill::create_bill(None, value.amount);
                        }
                    }
                    break;
                }
                "amount" => {
                    new_amout = Bill::get_amount();
                    match &value.name {
                        Some(inner_name) => {
                            *value = Bill::create_bill(Some(inner_name.to_owned()), new_amout);
                        }
                        None => {
                            *value = Bill::create_bill(None, new_amout);
                        }
                    }
                    break;
                }
                _ => {
                    println!("only name/amount can be edited. please enter name/amount:");
                    continue;
                }
            },
            Err(e) => {
                println!("error: {:?}", e);
                continue;
            }
        }
    }
    edited_hashmap
}

fn main() {
    let mut bills: HashMap<u32, Bill> = HashMap::new();
    let mut start = continue_input("generate bill");
    while start {
        let name: Option<String> = Bill::get_name();
        let amount = Bill::get_amount();
        bills.insert(generate_id(), Bill::create_bill(name, amount));
        start = continue_input("generate bill");
    }
    print_hashmap(&bills);
    if bills.len() > 0 {
        delete_bills(&mut bills)
    };
    if bills.len() > 0 {
        let edited_bills: HashMap<u32, Bill> = edit_bills(&mut bills);
        print_hashmap(&edited_bills);
    };
}
