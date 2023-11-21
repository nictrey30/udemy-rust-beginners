// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

use std::io;
#[derive(Debug, PartialEq)]
enum Names {
    First,
    Last,
}
fn name_type() -> Names {
    let mut input = String::new();
    loop {
        println!("Please enter the type of name you want: first/last");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: Names = match &input.trim()[..] {
            "first" => Names::First,
            "last" => Names::Last,
            "" => {
                println!("please enter something");
                input.clear();
                continue;
            }
            _ => {
                println!("only first/last");
                input.clear();
                continue;
            }
        };
        return input;
    }
}
fn validate_input(user_input: &str) -> bool {
    if user_input == "" {
        return false;
    } else {
        true
    }
}
fn validate_name(name_type: Names) -> String {
    let mut input = String::new();

    loop {
        if name_type == Names::First {
            println!("Please input your first name: ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if validate_input(&input.trim()[..]) == true {
                return input;
            } else {
                println!("empty input!");
                input.clear();
                continue;
            }
        } else if name_type == Names::Last {
            println!("Please input your last name: ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if validate_input(&input.trim()[..]) == true {
                return input;
            } else {
                println!("empty input!");
                input.clear();
                continue;
            }
        }
    }
}
fn main() {
    let name = validate_name(name_type());
    println!("name: {name}");
}
