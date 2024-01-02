// Topic: Iterator
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.

use std::io;

fn filtered_data(my_num: i32, vector: &Vec<i32>) -> Option<Vec<i32>> {
    let filtered_vec: Vec<&i32> = vector.iter().filter(|num| num == &&my_num).collect();
    let result: Vec<i32> = filtered_vec.iter().map(|num| **num).collect();
    if result.is_empty() {
        return None;
    }
    Some(result)
}

fn get_num() -> i32 {
    println!("please enter a number to find: ");
    let mut buffer = String::new();
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("incorrect data");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("please enter only numbers");
                buffer.clear();
                continue;
            }
        };
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 3, 4, 5, 3, -19, 3, 4];
    let filtered_data: Vec<i32> = filtered_data(get_num(), &data).unwrap_or_else(|| vec![]);
    println!(
        "initial vec: {:?} --> filtered_vec: {:?}",
        data, filtered_data
    );
}
