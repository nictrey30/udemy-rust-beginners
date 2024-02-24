// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

use std::io;

fn get_integer_input() -> i32 {
    let mut buffer = String::new();
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("incorrect data");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("please enter only integers");
                buffer.clear();
                continue;
            }
        }
    }
}

fn compare(num: i32) -> &'static str {
    match num {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    }
}

fn main() {
    let result: &str = compare(get_integer_input());
    println!("{result}");
}
