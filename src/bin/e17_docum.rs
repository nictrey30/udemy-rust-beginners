// Topic: Browsing standard library documentation

// Requirements:
// * Print a string in lowercase and uppercase
// Notes:
// * Utilize standard library functionality to transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str) to uppercase and lowercase
// * Try searching for: to_uppercase, to_lowercase
use std::io;

fn get_string() -> String {
    let mut buffer = String::new();
    println!("Please input a string: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        if buffer.trim().is_empty() {
            println!("empty values not allowed!");
            buffer.clear();
            continue;
        }
        return buffer;
    }
}

fn string_lowercase(string: &str) -> String {
    string.to_lowercase().to_owned()
}

fn string_uppercase(string: &str) -> String {
    string.to_uppercase().to_owned()
}

fn main() {
    let my_string = get_string();
    println!("{my_string} to lowercase: {}", string_lowercase(&my_string));
    println!("{my_string} to uppercase: {}", string_uppercase(&my_string));
}
