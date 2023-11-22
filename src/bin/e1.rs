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
    let mut input = String::new();
    loop {
        println!("please input an integer: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only integers allowed!");
                input.clear();
                continue;
            }
        };
        return input;
    }
}

fn choose_operation() -> Operations {
    let mut input = String::new();
    loop {
        println!("please input an operation(add/substract/multiply/divide): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        match input.to_lowercase().trim() {
            "add" => return Operations::Add,
            "substract" => return Operations::Substract,
            "multiply" => return Operations::Multiply,
            "divide" => return Operations::Divide,
            _ => {
                println!("please input only valid operations!");
                input.clear();
                continue;
            }
        }
    }
}

fn main() {
    let my_operation = choose_operation();
    println!("choose numbers: ");
    let a: i32 = read_input_num();
    let b: i32 = read_input_num();

    let result: f64 = match my_operation {
        Operations::Add => add(a, b) as f64,
        Operations::Substract => substract(a, b) as f64,
        Operations::Multiply => multiply(a, b) as f64,
        Operations::Divide => divide(a, b),
    };

    println!("The result of {a} {:?} {b} is {result}", my_operation);
}
