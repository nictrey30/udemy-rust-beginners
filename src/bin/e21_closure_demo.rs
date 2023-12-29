fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_fn(5, 6);
    println!("sum: {sum}");
    // closures must always be defineed within another function
    // let add = |a: i32, b: i32| -> i32 { a + b };
    let add = |a, b| a + b;
    println!("add with closures: {}", add(7, 6));
}
