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
        loop {
            match get_input("name") {
                Ok(string) => match string.trim() {
                    "" => {
                        println!("empty string not allowed.");
                        continue;
                    }
                    _ => return string,
                },
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
    fn print_bill(&self) {
        for bill in &self.inner {
            println!("{:?}", bill);
        }
    }
}

fn get_input(str: &str) -> io::Result<String> {
    let mut buffer = String::new();
    println!("get {}", str);
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {}
