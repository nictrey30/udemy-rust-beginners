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
                "" => println!("empty string not allowed."),
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
                Err(e) => println!("Please enter a number"),
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

// mutable reference to the structure Bills
fn add_bill_menu(bills: &mut Bills) {
    bills.add(Bill::new(Bill::get_name(), Bill::get_amount()));
    println!("bill added");
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1.Add bill");
        println!("2.View bill");
        println!("3.Quit");
        println!("Enter selection: ");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => break,
            _ => {
                println!("please choose 1/2/3 only");
                continue;
            }
        }
    }
}

fn main() {
    main_menu();
}
