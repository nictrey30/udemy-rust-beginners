enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat discount of 2"),
        Discount::Flat(amount) => println!("flat discount of {amount}"),
        // _ means ignore everything else
        _ => (),
    }
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        // match only when the price is 50
        Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
        // .. means any other fields ignore them
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
