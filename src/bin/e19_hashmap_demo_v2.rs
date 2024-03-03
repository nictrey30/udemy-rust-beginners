use std::{collections::HashMap, io};

// the contents will be a string, and we are using a struct just in case I want to specify exact contents later
#[derive(Debug)]
struct Contents {
    content: String,
}

impl Contents {
    fn new() -> Self {
        Self {
            content: get_contents(),
        }
    }
}

fn get_contents() -> String {
    let mut buffer = String::new();
    println!("Please input a string: ");

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("data error");
    }
    buffer.to_lowercase().trim().to_owned()
}

fn get_locker_num(lockers: &HashMap<i32, Contents>) -> i32 {
    // build the vector that contains the locker numbers that are already in the hashmap
    let mut locker_nums_vector: Vec<i32> = Vec::new();
    // check if the hashmap is empty
    if !lockers.is_empty() {
        for key in lockers.keys() {
            locker_nums_vector.push(*key);
        }
    }
    let mut buffer = String::new();
    println!("input an number between 0 and 100: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if locker_nums_vector.contains(&num) {
                    println!("num alread assigned to locker");
                    buffer.clear();
                    continue;
                }
                if num < 0 || num > 100 {
                    println!("{num} out of range (0..100)");
                    buffer.clear();
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("only numbers allowed.");
                buffer.clear();
                continue;
            }
        };
    }
}

fn continue_input() -> bool {
    let mut buffer = String::new();
    println!("Do you want to continue? (y/n): ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.to_lowercase().trim() {
            "y" => return true,
            "n" => return false,
            _ => {
                println!("input only (y/n)");
                buffer.clear();
                continue;
            }
        }
    }
}

fn main() {
    let mut continue_making_lockers = true;
    let mut lockers: HashMap<i32, Contents> = HashMap::new();
    while continue_making_lockers {
        let new_locker_num = get_locker_num(&lockers);
        lockers.insert(new_locker_num, Contents::new());
        continue_making_lockers = continue_input();
    }
}
