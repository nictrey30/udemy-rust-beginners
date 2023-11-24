// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
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

fn display_integers(num: u32) {
    for i in 1..num + 1 {
        println!("{i}");
    }
}

fn main() {
    display_integers(get_integer_value());
}
