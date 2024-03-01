// Topic: Result
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer is at least 21
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
use std::io;

struct Customer {
    name: Option<String>,
    age: i32,
}

impl Customer {
    fn new() -> Self {
        Self {
            name: get_name(),
            age: get_age(),
        }
    }

    fn check_age(&self) -> Result<bool, String> {
        match self.age > 21 {
            true => Ok(true),
            false => Err("cannot make purchase, under 21".to_owned()),
        }
    }
}

fn get_name() -> Option<String> {
    let mut buffer = String::new();
    println!("please input a name: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        // checking if the input has 0 length
        if buffer.trim().chars().count() == 0 {
            return None;
        }
        return Some(buffer.trim().to_lowercase().to_owned());
    }
}

fn get_age() -> i32 {
    let mut buffer = String::new();
    println!("please input an age: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if num < 0 || num > 120 {
                    println!("age out of range(0..120)");
                    buffer.clear();
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("age must be a number");
                buffer.clear();
                continue;
            }
        }
    }
}

fn print_age_verification(customer: &Customer) -> Result<(), String> {
    customer.check_age()?;
    match &customer.name {
        Some(inner_name) => println!("{} can make a purchase", inner_name),
        None => println!("can make a purchase"),
    }
    Ok(())
}

fn main() {
    let customer = Customer::new();
    if print_age_verification(&customer).is_err() {
        if !customer.name.is_none() {
            println!("{} cannot make purchase", customer.name.unwrap());
        } else {
            println!("cannot make a purchase");
        }
    }
}
