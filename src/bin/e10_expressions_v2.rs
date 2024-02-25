// Topic: Working with expressions
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
// Notes:
// * Use a boolean variable set to the result of an if..else expression to store whether the value is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message to print
use std::io;

fn check_variable(num: i32) -> bool {
    if num > 100 {
        true
    } else {
        false
    }
}

fn print_message(num: i32, result: bool) {
    if result {
        println!("{num} it's big");
    } else {
        println!("{num} it's small");
    }
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    println!("enter an integer:");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only integers allowed");
                buffer.clear();
                continue;
            }
        }
    }
}

fn main() {
    let my_number = get_input();
    let check_num: bool = check_variable(my_number);
    print_message(my_number, check_num);
}
