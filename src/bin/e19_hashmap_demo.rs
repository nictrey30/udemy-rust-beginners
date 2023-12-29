// hashmaps demo
#![allow(dead_code, unused_variables, unused_imports)]
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Contents {
    content: String,
}

fn get_contents() -> String {
    println!("locker contents: ");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.to_lowercase().trim() {
            "" => {
                println!("err: empty input!");
                continue;
            }
            _ => return input.to_lowercase().trim().to_owned(),
        };
    }
}

fn get_num() -> u32 {
    let mut input = String::new();
    loop {
        input.clear();
        println!("(only positive integers between 1..100): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.trim().parse() {
            Ok(num) => {
                if num < 1 {
                    println!("no cannot be < 1");
                    continue;
                } else if num > 100 {
                    println!("no cannot be > 100");
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("only numbers between 1..100 allowed!");
                continue;
            }
        };
    }
}

fn print_locker_nums(vector: &Vec<u32>) -> String {
    let mut vector_nums_string = String::new();
    for i in vector {
        vector_nums_string.push_str(&i.to_string());
        vector_nums_string.push_str(", ");
    }
    vector_nums_string
}

fn create_lockers(num: u32) -> HashMap<u32, Contents> {
    let mut locker_nums: Vec<u32> = Vec::new();
    let mut lockers: HashMap<u32, Contents> = HashMap::new();
    for i in 0..num {
        loop {
            // print the current nums in the vector
            if i != 0 {
                println!(
                    "current num in lockers: {}",
                    print_locker_nums(&locker_nums)
                )
            };
            // check if num input already in vector
            print_locker_nums(&locker_nums);
            println!("assign locker no:");
            let num = get_num();
            if locker_nums.contains(&num) {
                println!("num already in locker_nums!");
                continue;
            } else {
                locker_nums.push(num);
                lockers.insert(
                    num,
                    Contents {
                        content: get_contents(),
                    },
                );
                break;
            }
        }
    }
    lockers
}

fn main() {
    println!("How many lockers do you want?");
    let num_of_lockers = get_num();
    let all_lockers: HashMap<u32, Contents> = create_lockers(num_of_lockers);
    for (k, v) in all_lockers {
        println!("{k}: {:?}", v);
    }
}
