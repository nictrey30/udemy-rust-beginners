// Project 1: Interactive bill manager
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
#![allow(dead_code, unused_variables, unused_imports)]

use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill {
    fn new(name: String, amount: f64) -> Self {
        Self { name, amount }
    }

    fn get_name() -> String {
        println!("name: ");
        loop {
            let name = get_input();
            match name.trim() {
                "" => {
                    println!("empty string not allowed.");
                    continue;
                }

                _ => return name,
            }
        }
    }

    fn get_amount() -> f64 {
        println!("amount: ");
        loop {
            let amount = get_input();
            match amount.parse() {
                Ok(num) => return num,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            }
        }
    }
}

// the reason the vec is in struct is if I decide to change the inner value from a vector to something elese, I will only need to change it in the structure
struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }
    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }
    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter your data again");
    }
    buffer.trim().to_owned()
}

fn continue_input() -> bool {
    println!("press n to quit");
    match get_input().trim() {
        "n" => return false,
        _ => return true,
    };
}

fn main() {
    let mut bills: Bills;
    bills = Bills::new();
    let mut cont: bool = continue_input();
    while cont {
        bills.add(Bill::new(Bill::get_name(), Bill::get_amount()));
        cont = continue_input();
    }
    println!("finished");
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}