// Topic: Basic arithmetic

use std::io;
#[derive(Debug)]
enum Operations {
    Add,
    Substract,
    Multiply,
    Divide,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn substract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, mut b: i32) -> f64 {
    while b == 0 {
        println!("cannot divide by");
        b = read_input_num();
    }
    a as f64 / b as f64
}

fn read_input_num() -> i32 {
    let mut buffer = String::new();
    loop {
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

fn choose_operation() -> Operations {
    let mut input = String::new();
    println!("please input an operation(add/sub/mult/div): ");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        match input.to_lowercase().trim() {
            "add" => return Operations::Add,
            "sub" => return Operations::Substract,
            "mult" => return Operations::Multiply,
            "div" => return Operations::Divide,
            _ => {
                println!("please input only valid operation(add/sub/mult/div)");
                input.clear();
                continue;
            }
        }
    }
}

fn main() {
    println!("enter 1st number: ");
    let a: i32 = read_input_num();
    println!("enter 2nd number: ");
    let b: i32 = read_input_num();
    let my_operation = choose_operation();
    let result: f64 = match my_operation {
        Operations::Add => add(a, b) as f64,
        Operations::Substract => substract(a, b) as f64,
        Operations::Multiply => multiply(a, b) as f64,
        Operations::Divide => divide(a, b),
    };

    println!("The result of: {a} {:?} {b} = {result}", my_operation);
}
