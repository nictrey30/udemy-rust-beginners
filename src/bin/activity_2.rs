// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

use std::io;

fn get_input() -> i32 {
    let mut buffer = String::new();
    loop {
        println!("enter a number: ");
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("please enter your data again");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                buffer.clear();
                println!("please enter only integers");
                continue;
            }
        }
    }
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_num(a: i32) {
    println!("result: {:?}", a);
}

fn main() {
    let a: i32 = get_input();
    let b: i32 = get_input();
    display_num(sum(a, b));
}
