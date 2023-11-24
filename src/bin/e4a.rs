// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

use rand;
use std::{collections::HashMap, io};

#[derive(Debug)]
enum Coin {
    Head,
    Tail,
}

fn generate_flip() -> Coin {
    match rand::random::<bool>() {
        true => Coin::Head,
        false => Coin::Tail,
    }
}

fn coin_flips_vector(num: u32) -> Vec<Coin> {
    let mut coin_flips: Vec<Coin> = Vec::new();
    for _i in 0..num {
        coin_flips.push(generate_flip())
    }
    coin_flips
}

fn num_of_flips() -> u32 {
    let mut input = String::new();
    println!("how many flips do you want to calculate?");
    loop {
        println!("Please enter a number between 1 and 100: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        let input: u32 = match input.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("error: integer > 100");
                    input.clear();
                    continue;
                } else if num == 0 {
                    println!("error: not 0");
                    input.clear();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("only positive integers allowed");
                input.clear();
                continue;
            }
        };
        return input;
    }
}

// return a hashmap containing how many flips are for heads/tails
fn calculate_total_flips(vector: &Vec<Coin>) -> HashMap<String, u32> {
    let mut flips = HashMap::new();
    for i in vector {
        match i {
            Coin::Head => {
                let count = flips.entry("heads".to_owned()).or_insert(0);
                *count += 1;
            }
            Coin::Tail => {
                let count = flips.entry("tails".to_owned()).or_insert(0);
                *count += 1;
            }
        }
    }
    flips
}

// return a hashmap calculating the probabilities that takes the no_flips for heads/tails
fn calculate_probability(flips: &HashMap<String, u32>, no_flips: u32) -> HashMap<String, f64> {
    let mut flips_probabilities: HashMap<String, f64> = HashMap::new();
    let heads_probability = flips_probabilities
        .entry("heads_probability".to_owned())
        .or_insert(0.0);
    *heads_probability =
        (flips.get("heads").copied().unwrap_or(0) as f64 / no_flips as f64) * 100.00;
    let tails_probability = flips_probabilities
        .entry("tails_probability".to_owned())
        .or_insert(0.0);
    *tails_probability =
        (flips.get("tails").copied().unwrap_or(0) as f64 / no_flips as f64) * 100.00;
    flips_probabilities
}

fn main() {
    // chose a no_flips between 0 and 100
    let no_flips: u32 = num_of_flips();
    // return a vector containing the no_flips flips
    let vector_of_flips = coin_flips_vector(no_flips);
    println!("The vector of coin flips: ");
    for i in &vector_of_flips {
        println!("{:?}", i);
    }
    // return a hashmap containing how many flips are for heads/tails
    let total_flips = calculate_total_flips(&vector_of_flips);
    println!("Total flips: {:?}", total_flips);
    // return a hashmap calculating the probabilities that takes the no_flips for heads/tails
    let hash_probability: HashMap<String, f64> = calculate_probability(&total_flips, no_flips);
    for (k, v) in hash_probability {
        println!("{k}: {:.2}", v);
    }
}
