#![allow(dead_code, unused_variables, unused_imports)]

mod greet {
    pub fn hello() {
        println!("hello");
    }

    pub fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

use greet::hello;

fn main() {
    hello();
    greet::goodbye();
    println!("add result: {}", math::add(1, 2));
}
