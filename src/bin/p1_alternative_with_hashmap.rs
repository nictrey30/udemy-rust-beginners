// Project 1: Interactive bill manager
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
#![allow(dead_code, unused_variables, unused_imports)]

use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
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
            match name {
                Some(inner_name) => return inner_name,
                None => {
                    println!("empty names not allowed");
                    continue;
                }
            }
        }
    }

    fn get_amount() -> f64 {
        loop {
            println!("amount: ");
            let amount = get_input();
            match amount {
                Some(num) => match num.parse() {
                    Ok(inner_num) => return inner_num,
                    Err(_) => {
                        println!("only numbers allowed");
                        continue;
                    }
                },
                None => {
                    println!("empty data not allowed");
                    continue;
                }
            }
        }
    }
}

struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        // the reasone I'm cloning the bill name because insert will move the bill name, but the bill needs to have an owned name, but we don't want to move it into the key part of the hashmap
        self.inner.insert(bill.name.clone(), bill);
    }

    fn get_all(&self) -> Vec<Bill> {
        let mut bills: Vec<Bill> = vec![];
        // when iterating over values of a hashmap, bill is always borrowed, but i need to push an owned bill into the vector, therefore we need to derive clone for Bills struct
        for bill in self.inner.values() {
            bills.push(bill.clone());
        }
        bills
    }

    // removing a bill based on the name of the bill
    // if we cannot the bill based on the name return false, indicating that the removing failed
    fn remove(&mut self, name: &str) -> bool {
        // .remove(&key) will return an Option. If we want to disregard the return value from Some we use is_some()
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str) -> bool {
        // get_mut() Returns a mutable reference to the value corresponding to the key. It returns an Option
        match self.inner.get_mut(name) {
            Some(bill) => {
                let amount = Bill::get_amount();
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

// mutable reference to the structure Bills
fn add_bill_menu(bills: &mut Bills) {
    println!("bill name: ");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = Bill::get_amount();
    bills.add(Bill::new(name, amount));
    println!("bill added");
}

fn remove_bill_menu(bills: &mut Bills) {
    // viwew the bills
    view_bill_menu(bills);
    println!("==> remove key by name: ");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    match bills.remove(&name) {
        true => {
            println!("bills updated:");
            view_bill_menu(bills);
        }
        false => println!("bill \"{name}\" not found"),
    }
}

// update the amount of a bill
fn update_bill_menu(bills: &mut Bills) {
    // viwew the bills
    view_bill_menu(bills);
    println!("==> update key by name: ");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    if bills.update(&name) {
        println!("updated");
        view_bill_menu(bills);
    } else {
        println!("bill \"{name}\" not found");
    }
}

fn view_bill_menu(bills: &Bills) {
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
        println!("3.Delete bill");
        println!("4.Edit bill");
        println!("5.Quit");
        println!("Enter selection: ");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = match get_input() {
            Some(inner) => inner,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bill_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            "5" => break,
            _ => {
                println!("please choose 1/2/3/4/5 only");
                continue;
            }
        }
    }
}

fn main() {
    main_menu();
}
