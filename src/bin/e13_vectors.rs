// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

use std::io;

fn get_no_elements_value() -> u32 {
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

fn generate_vector(num: u32) -> Vec<u32> {
    let mut new_vector: Vec<u32> = Vec::new();
    for i in 1..num + 1 {
        new_vector.push(i * 10);
    }
    new_vector
}

fn int_el_to_string(num: u32) -> String {
    match num {
        10 => "ten".to_owned(),
        20 => "twenty".to_owned(),
        30 => "thirty".to_owned(),
        40 => "fourty".to_owned(),
        50 => "fifty".to_owned(),
        60 => "sixty".to_owned(),
        70 => "seventy".to_owned(),
        80 => "eighty".to_owned(),
        90 => "ninety".to_owned(),
        100 => "one hundred".to_owned(),
        _ => "error: input out of range".to_owned(),
    }
}

// returns a vector of Strings based on an integer value
fn transform_vector_to_string(vector: &Vec<u32>) -> Vec<String> {
    let mut vector_strings: Vec<String> = Vec::new();
    for i in vector {
        vector_strings.push(int_el_to_string(*i));
    }
    vector_strings
}

fn print_vector(vec: &Vec<String>) {
    for i in vec {
        println!("{i}");
    }
}

fn main() {
    let num_elemets: u32 = get_no_elements_value();
    let my_vector = generate_vector(num_elemets);
    println!("{:?}", my_vector);
    let transformed_vector = transform_vector_to_string(&my_vector);
    print_vector(&transformed_vector);
}
