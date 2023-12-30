#![allow(dead_code, unused_variables, unused_imports)]
// Topic: Map combinator
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

use std::io;

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn get_input() -> String {
    println!("input user:");
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter data again");
    }
    buffer.to_lowercase().trim().to_owned()
}

fn main() {
    let name = get_input();
    let user = find_user(&name).map(|user_id| User { user_id, name });
    match user {
        Some(inner_user) => println!("{:?}", inner_user),
        None => println!("user not found"),
    }
}
