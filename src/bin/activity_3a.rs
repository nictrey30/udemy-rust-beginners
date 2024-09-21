// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

use std::io;

fn print_message(value: bool) {
    if value {
        println!("hello");
    } else {
        println!("goodbye");
    }
}

fn get_value() -> bool {
    let mut buf = String::new();
    loop {
        println!("true or false: ");
        while io::stdin().read_line(&mut buf).is_err() {
            println!("please enter the data again");
        }
        match buf.to_lowercase().trim() {
            "true" => return true,
            "false" => return false,
            _ => {
                println!("only true/false values allowed");
                buf.clear();
            }
        }
    }
}

fn main() {
    print_message(get_value());
}
