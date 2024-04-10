use std::io;

// * Use a struct to store at least the age of a customer
struct Customer {
    age: i32,
}

fn get_age() -> i32 {
    let mut buffer = String::new();
    println!("please input an age: ");
    loop {
        buffer.clear();
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if num <= 0 || num >= 120 {
                    println!("age out of range(0..120)");
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("please input a number!");
            }
        }
    }
}

// * Use a function to determine if a customer can make a restricted purchase, Return a result from the function
fn validate_purchase(customer: &Customer) -> Result<(), String> {
    // * Restricted purchases require that the age of the customer is at least 21
    if customer.age < 21 {
        // * The Err variant should detail the reason why they cannot make a purchase
        return Err("under 21. cannont make purchase".to_string());
    }
    Ok(())
}

fn main() {
    let customer = Customer { age: get_age() };
    match validate_purchase(&customer) {
        Ok(()) => println!("can make a purchase"),
        Err(e) => println!("{}", e),
    }
}
