// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
#![allow(dead_code, unused_variables, unused_imports)]
use std::io;

#[derive(Debug, Clone)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

impl TicketType {
    fn get_name() -> String {
        let mut value = String::new();
        loop {
            value.clear();
            println!("name on ticket: ");
            io::stdin()
                .read_line(&mut value)
                .expect("failed to read line");
            if value.trim() == "" {
                println!("empty values not allowed.");
                continue;
            }
            return value.trim().to_lowercase().to_owned();
        }
    }

    fn get_type() -> Self {
        let name = Self::get_name();
        let mut value = String::new();
        loop {
            value.clear();
            println!("ticket type: ");
            io::stdin()
                .read_line(&mut value)
                .expect("failed to read line");
            match value.to_lowercase().trim() {
                "backstage" => return Self::Backstage(name),
                "vip" => return Self::Vip(name),
                "standard" => return Self::Standard,
                _ => {
                    println!("only backstage/vip/standard inputs allowed");
                    continue;
                }
            };
        }
    }
}

#[derive(Debug)]
struct Tickets {
    ticket_type: TicketType,
    price: f64,
}

impl Tickets {
    fn create_ticket(ticket_type: TicketType) -> Self {
        let price = get_ticket_price();
        Self { ticket_type, price }
    }
}

fn get_ticket_price() -> f64 {
    let mut value = String::new();
    println!("input an number between 1 and 100: ");
    loop {
        value.clear();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: f64 = match value.trim().parse() {
            Ok(num) => {
                if num == 0.0 {
                    println!("0 not allowed");
                    continue;
                } else if num > 100.0 {
                    println!("ticket cannot be more expensive than 100");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("only numbers between 1..100 allowed.");
                continue;
            }
        };
        return f64::from(value);
    }
}

fn get_num() -> u32 {
    let mut value = String::new();
    println!("input an number between 1 and 5: ");
    loop {
        value.clear();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: u32 = match value.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("0 not allowed");
                    continue;
                } else if num > 5 {
                    println!("no more than 5");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("only numbers between 1..5 allowed.");
                continue;
            }
        };
        return value;
    }
}

fn tickets_list(num: u32) -> Vec<Tickets> {
    let mut tickets: Vec<Tickets> = Vec::new();
    for _i in 0..num {
        tickets.push(Tickets::create_ticket(TicketType::get_type()));
    }
    tickets
}

fn ticket_probabilities(vector: &Vec<Tickets>) {
    let mut standard = 0;
    let mut backstage = 0;
    let mut vip = 0;
    for i in vector {
        println!("{:?}", i);
        match i.ticket_type.clone() {
            TicketType::Vip(_name) => {
                vip += 1;
            }
            TicketType::Backstage(_name) => {
                backstage += 1;
            }
            TicketType::Standard => {
                standard += 1;
            }
        };
    }
    println!(
        "standard: {} tickets, backstage: {} tickets, vip: {} tickets",
        standard, backstage, vip
    );
}

fn main() {
    println!("Input no of tickets:");
    let no_tickets = get_num();
    let my_tickets: Vec<Tickets> = tickets_list(no_tickets);
    // display how many tickets of each type in procentages are
    ticket_probabilities(&my_tickets);
}
