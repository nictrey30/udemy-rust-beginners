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
            println!("name: ");
            match get_input() {
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

    fn get_amount() -> f64 {
        loop {
            println!("amount: ");
            match get_input() {
                Ok(string) => match string.parse() {
                    Ok(num) => return num,
                    Err(e) => {
                        println!("error: {:?}", e);
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
    fn print_bills(&self) {
        for bill in &self.inner {
            println!("{:?}", bill);
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn continue_input(str: &str) -> bool {
    let mut answer = String::new();
    loop {
        answer.clear();
        println!("{} y/n", str);
        let answer = match get_input() {
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

fn main() {
    let mut bills: Bills;
    bills = Bills::new();
    loop {
        match continue_input("add bill") {
            true => {
                bills.add(Bill::new(Bill::get_name(), Bill::get_amount()));
            }
            false => {
                println!("finished");
                bills.print_bills();
                break;
            }
        }
    }
}
