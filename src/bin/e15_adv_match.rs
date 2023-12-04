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
use std::io;

#[derive(Debug)]
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
            println!("name: ");
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
        let mut input_ticket_type: String = Self::get_name();
        loop {
            match input_ticket_type.trim() {
                "backstage" => return Self::Backstage(Self::get_name()),
                "vip" => return Self::Vip(Self::get_name()),
                "standard" => return Self::Standard,
                _ => {
                    println!("only backstage/vip/standard inputs allowed");
                    input_ticket_type.clear();
                    continue;
                }
            };
        }
    }
}

struct Tickets {
    ticket_type: TicketType,
    price: f64,
}

impl Tickets {
    fn create_ticker(ticket_type: TicketType) -> Self {
        let price
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

// fn tickets_list(num: u32) -> Vec<Tickets> {
//     let mut tickets: Vec<Tickets> = Vec::new();
//     for i in 0..num {
//         tickets.push()
//     }
// }

fn main() {
    let no_tickets = get_num();
}
