// Topic: User input
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to the console indicating which action will be taken
// Example: If the user types in "shutdown" a message should display such as "shutting down"
// * If the keyword entered does not exist, an appropriate error message should be displayed
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type Reboot, reboot, REBOOT, etc.)

use std::io;

#[derive(Debug)]
enum States {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

use States::*;

fn get_user_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    // .trim() -> &str, to_lowercase -> String
    return Ok(buffer.trim().to_lowercase());
}

fn validate_input(input: &str) -> Result<States, String> {
    match input {
        "off" => return Ok(Off),
        "sleep" => return Ok(Sleep),
        "hibernate" => return Ok(Hibernate),
        "shutdown" => return Ok(Shutdown),
        "reboot" => return Ok(Reboot),
        "" => Err("empty input".to_owned()),
        _ => Err("input does not exist.".to_owned()),
    }
}

fn return_state() -> States {
    loop {
        println!("user input: off/sleep/reboot/shutdown/hibernate: ");
        match get_user_input() {
            Ok(input) => match validate_input(&input) {
                Ok(state) => return state,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            },
            Err(e) => {
                println!("error: {:?}", e);
                continue;
            }
        }
    }
}

fn print_state(state: &States) {
    match state {
        Off => println!("PC is off..."),
        Reboot => println!("rebooting..."),
        Hibernate => println!("hibernating..."),
        Sleep => println!("sleeping mode..."),
        Shutdown => println!("shutting down..."),
    }
}

fn main() {
    let user_choice: States = return_state();
    print_state(&user_choice);
}
