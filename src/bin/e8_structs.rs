// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
use std::cmp::Ordering;
use std::io;
// import the macros needed
use strum_macros::{EnumString, EnumVariantNames};
// You need to import the trait, to have access to VARIANTS
use strum::VariantNames;
const SIZES: &'static [i32] = &[330, 500, 750, 1000, 1500];

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum Flavors {
    Cherry,
    Diet,
    Zero,
    Regular,
}
#[derive(Debug)]
struct Drinks {
    flavor: Flavors,
    size: i32,
}

fn display_flavors() {
    println!("The available flavors are: ");
    for i in Flavors::VARIANTS {
        println!("{}", i.to_lowercase());
    }
}

fn validate_size(num: &i32) -> bool {
    match num.cmp(&0) {
        Ordering::Less => {
            return false;
        }
        Ordering::Equal => {
            println!("zero values not allowed");
            return false;
        }
        Ordering::Greater => {
            for i in SIZES {
                if num == i {
                    return true;
                }
            }
            false
        }
    }
}

fn input_size() -> i32 {
    let mut size_in_ml = String::new();
    loop {
        println!("_______________________________________________________");
        println!("please choose size from: 330/ 500/ 750/ 1000/ 1500ml:");
        size_in_ml.clear();
        io::stdin()
            .read_line(&mut size_in_ml)
            .expect("failed to read line.");
        let size_in_ml: i32 = match size_in_ml.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only integers allowed");
                continue;
            }
        };
        if validate_size(&size_in_ml) {
            return size_in_ml;
        } else {
            if size_in_ml != 0 {
                println!("value not in allowed values");
            }
            continue;
        }
    }
}

fn input_flavor() -> Flavors {
    let mut flavor = String::new();
    loop {
        println!("____________________________");
        println!("please choose a valid flavor");
        display_flavors();
        io::stdin()
            .read_line(&mut flavor)
            .expect("failed to read line");
        match flavor.to_lowercase().trim() {
            "cherry" => return Flavors::Cherry,
            "diet" => return Flavors::Diet,
            "regular" => return Flavors::Regular,
            "zero" => return Flavors::Zero,
            _ => {
                println!("only valid flavors allowed");
                flavor.clear();
                continue;
            }
        };
    }
}

fn make_drink(size: i32, flavor: Flavors) -> Drinks {
    Drinks { flavor, size: size }
}

fn main() {
    let size = input_size();
    let flavor = input_flavor();
    let drink = make_drink(size, flavor);
    println!("The drink you chose: {:?}", drink);
}
