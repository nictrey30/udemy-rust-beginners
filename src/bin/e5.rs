use std::io;

fn get_integer_value() -> u32 {
    let mut buffer = String::new();
    println!("input an integer value between 1 and 10: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("please enter your data again");
        }
        let value: u32 = match buffer.trim().parse() {
            Ok(num) => {
                if num == 0 || num > 10 {
                    println!("numbers between 1 and 10 only");
                    buffer.clear();
                    continue;
                }
                num
            }
            Err(_) => {
                println!("only u32 allowed");
                buffer.clear();
                continue;
            }
        };
        return value;
    }
}

fn display_integers(num: u32) {
    println!("the numbers are: ");
    for i in 1..num + 1 {
        println!("{i}");
    }
}

fn main() {
    display_integers(get_integer_value());
}
