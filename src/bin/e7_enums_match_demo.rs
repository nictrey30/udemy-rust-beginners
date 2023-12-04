#![allow(dead_code, unused_variables, unused_imports)]

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {other}"),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount amount of {:?}", amount),
        _ => (),
    };

    let concert_ticket = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert_ticket {
        Ticket { price: 50, event } => println!("event @ 50: {:?}", event),
        // we are only concerned with the price, ignore any other field
        Ticket { price, .. } => println!("price: {:?}", price),
    }
}