// Topic: Functions
// Program requirements:
// * Displays your first and last name

// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal
use std::io;

fn get_name(name: &str) -> String {
    let mut buffer = String::new();
    println!("please input {name}: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        // checking if the input has 0 length
        match buffer.trim() {
            "" => {
                println!("input cannot be empty");
            }
            _ => return buffer.trim().to_lowercase().to_owned(),
        }
    }
}

fn display_name(name: &String) {
    println!("{}", name);
}

fn main() {
    let first_name: String = get_name("first name");
    let last_name: String = get_name("last name");
    println!("first_name:");
    display_name(&first_name);
    println!("last_name:");
    display_name(&last_name);
}
