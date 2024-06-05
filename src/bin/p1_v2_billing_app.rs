// Project 1: Interactive bill manager
// User stories:
// * L1: I want to add bills, including the name and amount owed. L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills. L3: I want to go back if I change my mind.
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it throughout your program.

use std::io;

struct Bill {
    name: String,
    amount: f64,
}

// the reason I created a struct insead of using a vec directly is if I decide to change this inner value from a vec to something else I will need to change it in one place only
struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }
    fn add(&mut self, bill: Bill) {
        self.inner.push(bill);
    }
    // get_all will return a borrowed vector
    fn get_all(&self) -> &Vec<Bill> {
        &self.inner
    }
}

// * Create a function just for retrieving user input, and reuse it throughout your program.
fn get_input() -> String {
    let mut buffer = String::new();
    // read_line returns a Result, but my function returns a String, so I will continue the loop until a String is returned
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter your data again");
    }
    buffer.trim().to_owned()
}

// we can call show() only from the main_menu()
fn main_menu() {
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("");
        println!("Enter selection:");
    }
    let mut bills = Bills::new();
    // main input loop for the main menu
    loop {
        show();
        let input = get_input();
        // matching with the help of as_str() because get_input() returnes an owned String
        // matching against borrowed strings
    }
}

fn main() {}
