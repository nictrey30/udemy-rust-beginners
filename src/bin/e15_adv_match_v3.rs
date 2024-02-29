use std::io;

#[derive(Debug)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

impl TicketType {
    fn new() -> Self {
        let mut buffer = String::new();
        println!("ticket type standard/vip/backstage: ");
        loop {
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.to_lowercase().trim() {
                "backstage" => {
                    let name = get_name();
                    return Self::Backstage(name);
                }
                "vip" => {
                    let name = get_name();
                    return Self::Vip(name);
                }
                "standard" => return Self::Standard,
                _ => {
                    println!("standard/vip/backstage only");
                    buffer.clear();
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Ticket {
    ticket_type: TicketType,
    price: f64,
}

impl Ticket {
    fn new() -> Self {
        let mut buffer = String::new();
        let ticket_type = TicketType::new();
        println!("ticket price: ");
        loop {
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            let price: f64 = match buffer.trim().parse() {
                Ok(num) => {
                    if num < 0.0 || num > 100.0 {
                        println!("price must be between 0 and 100");
                        continue;
                    }
                    num
                }
                Err(_) => {
                    println!("only numbers between 0..100 allowed.");
                    buffer.clear();
                    continue;
                }
            };
            return Self { ticket_type, price };
        }
    }
}

fn get_name() -> String {
    let mut buffer = String::new();
    println!("name on ticket: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim() {
            "" => {
                println!("empty data. try again");
                buffer.clear();
                continue;
            }
            _ => return buffer,
        }
    }
}

fn get_num_tickets() -> i32 {
    let mut buffer = String::new();
    println!("enter how many tickets do you want to add(1 to 10)");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if num <= 0 || num > 10 {
                    println!("num out of range(1..10)");
                    buffer.clear();
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("only numbers between 1..10 allowed.");
                buffer.clear();
                continue;
            }
        }
    }
}

fn print_tickets(vector: &Vec<Ticket>) {
    println!("Tickets: ");
    for ticket in vector {
        // * Use a match expression while iterating the vector to print the ticket info
        // * Use an enum for the tickets with data associated with each variant
        match &ticket.ticket_type {
            TicketType::Backstage(name) => {
                println!("backstage ticket for {name}, price {}", ticket.price)
            }
            TicketType::Vip(name) => println!("vip ticket for {name}, price {}", ticket.price),
            TicketType::Standard => println!("standard ticket , price {}", ticket.price),
        }
        println!("____________");
    }
}

fn main() {
    let mut tickets: Vec<Ticket> = Vec::new();
    let num_of_tickets = get_num_tickets();
    for _i in 0..num_of_tickets {
        // * Create one of each ticket and place into a vector
        let ticket = Ticket::new();
        tickets.push(ticket);
    }
    print_tickets(&tickets);
}
