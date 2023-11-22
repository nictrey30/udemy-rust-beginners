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
    let mut input = String::new();
    loop {
        println!("please input a true/false bool value: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        let input: bool = match input.to_lowercase().trim() {
            "true" => true,
            "false" => false,
            _ => {
                println!("only true/false allowed!");
                input.clear();
                continue;
            }
        };
        return input;
    }
}

fn display_msg(user_bool_input: bool) {
    if user_bool_input == true {
        println!("hello");
    } else {
        println!("goodbye");
    }
}

fn main() {
    display_msg(read_user_bool_val());
}
