// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop
use std::io;

fn get_integer_value() -> u32 {
    let mut value = String::new();
    println!("input an integer value between 1 and 10: ");
    loop {
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: u32 = match value.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("0 not allowed");
                    value.clear();
                    continue;
                } else if num > 10 {
                    println!("no more than 10");
                    value.clear();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("only u32 allowed");
                value.clear();
                continue;
            }
        };
        return value;
    }
}

fn display_countdown(mut num: u32) {
    println!("countdown: ");
    while num >= 1 {
        println!("{num}");
        num -= 1;
    }
    println!("done!");
}

fn main() {
    display_countdown(get_integer_value());
}
