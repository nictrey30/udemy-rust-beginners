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

fn read_user_bool_val() -> bool {
    let mut buffer = String::new();
    loop {
        println!("please input a true/false bool value: ");
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("please enter your data again");
        }
        let input: bool = match buffer.to_lowercase().trim() {
            "true" => true,
            "false" => false,
            _ => {
                println!("only true/false allowed!");
                buffer.clear();
                continue;
            }
        };
        return input;
    }
}

fn display_msg(user_bool_input: bool) {
    if user_bool_input == true {
        println!("hello");
        return ();
    }
    println!("goodbye");
}

fn main() {
    display_msg(read_user_bool_val());
}
