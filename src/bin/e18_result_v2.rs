// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
use std::io;

#[derive(Debug)]
struct Customer {
    name: Option<String>,
    age: u32,
}

impl Customer {
    fn create_customer() -> Self {
        let cust_name = get_name();
        let age = get_age();
        match cust_name {
            Some(name) => {
                return Self {
                    name: Some(name),
                    age,
                }
            }
            None => {
                return Self { name: None, age };
            }
        }
    }
}

fn get_name() -> Option<String> {
    let mut input = String::new();
    println!("customer's name(can be blank): ");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line.");
    match input.trim() {
        "" => return None,
        name => return Some(name.to_owned()),
    }
}

fn get_age() -> u32 {
    let mut input = String::new();
    loop {
        input.clear();
        println!("customer's age(only positive integers between 1..100): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.trim().parse() {
            Ok(num) => {
                if num < 1 {
                    println!("age cannot be < 1");
                    continue;
                } else if num > 100 {
                    println!("age cannot be > 100");
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("only numbers between 1..100 allowed!");
                continue;
            }
        };
    }
}

fn age_verify(customer: &Customer) -> Result<(), String> {
    match customer.age < 21 {
        true => return Err("cannot make a purchase".to_owned()),
        false => return Ok(()),
    }
}

fn print_result(customer: &Customer) -> Result<(), String> {
    age_verify(customer)?;
    match customer.name.clone() {
        Some(name) => println!("{name} can make a purchase"),
        None => println!("can make a purchase"),
    }
    Ok(())
}

fn main() {
    let customer1 = Customer::create_customer();
    print_result(&customer1).expect("cannot make purchase");
}
