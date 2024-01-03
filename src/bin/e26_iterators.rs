#![allow(dead_code, unused_variables, unused_imports)]
// Topic: Iterator

use std::io;

// * Triple the value of each item in a vector.
fn multiply_elements(my_num: i32, vector: &Vec<i32>) -> Option<Vec<i32>> {
    if vector.is_empty() {
        return None;
    }
    let result = vector.iter().map(|num| num * my_num).collect();
    Some(result)
}

// * Filter the data to only include values > 10.
fn filter_elements(my_num: i32, vector: &Vec<i32>) -> Option<Vec<i32>> {
    if vector.is_empty() {
        return None;
    }
    let result: Vec<i32> = vector
        .clone()
        .iter()
        .map(|num| *num)
        .filter(|num| num > &my_num)
        .collect();
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

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn filter_elements_found() {
        let data: Vec<i32> = vec![1, 2, 3, 3];
        let result = filter_elements(2, &data);
        let expectation = vec![3, 3];
        assert_eq!(
            result.unwrap_or_else(|| vec![]),
            expectation,
            "should be [3, 3]"
        );
    }
    #[test]
    fn filter_elements_not_found() {
        let data: Vec<i32> = vec![1, 2, 3, 3];
        let result = filter_elements(7, &data);
        let expectation = vec![];
        assert_eq!(
            result.unwrap_or_else(|| vec![]),
            expectation,
            "should be []"
        );
    }

    #[test]
    fn multiply_elements_test() {
        let data: Vec<i32> = vec![1, 2, 3, 3];
        let result = multiply_elements(3, &data).unwrap_or_else(|| vec![]);
        let expect = vec![3, 6, 9, 9];
        assert_eq!(result, expect, "should be vec![3, 6, 9, 9]");
    }
    #[test]
    fn multiply_elements_test_none() {
        let data: Vec<i32> = vec![];
        let result = multiply_elements(3, &data);
        let expect = None;
        assert_eq!(result, expect, "should be None");
    }
}
