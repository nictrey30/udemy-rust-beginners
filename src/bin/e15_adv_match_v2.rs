// Topic: advanced match
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

use std::io;

#[derive(Debug)]
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

impl Ticket {
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
                    }
                    if num > 100.0 {
                        println!("ticket cannot be more expensive than 100");
                        continue;
                    }
                    num
                }
                Err(_) => {
                    println!("only numbers between 1..100 allowed.");
                    continue;
                }
            };
            return f64::from(value);
        }
    }
    // get the Name for Backstage and Vip Tickets
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
    // create ticket
    fn create_ticket() -> Self {
        let name = Self::get_name();
        let mut value = String::new();
        loop {
            value.clear();
            println!("ticket type: ");
            io::stdin()
                .read_line(&mut value)
                .expect("failed to read line");
            match value.to_lowercase().trim() {
                "backstage" => return Self::Backstage(Self::get_ticket_price(), name),
                "vip" => return Self::Vip(Self::get_ticket_price(), name),
                "standard" => return Self::Standard(Self::get_ticket_price()),
                _ => {
                    println!("only backstage/vip/standard inputs allowed");
                    continue;
                }
            };
        }
    }
}

// input num of tickets..max 5
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
                }
                if num > 5 {
                    println!("no more than 5");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("only numbers between 1..5 allowed.");
                continue;
            }
        };
        return value;
    }
}

fn tickets_list(num: u32) -> Vec<Ticket> {
    let mut tickets: Vec<Ticket> = Vec::new();
    for _i in 0..num {
        tickets.push(Ticket::create_ticket());
    }
    tickets
}

fn main() {
    let num_tickets = get_num();
    let my_tickets = tickets_list(num_tickets);
    for i in &my_tickets {
        println!("{:?}", i);
    }
}
