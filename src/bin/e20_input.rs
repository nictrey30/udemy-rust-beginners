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

fn get_user_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(buffer.trim().to_lowercase().to_owned());
}

fn validate_input(input: &str) -> Result<States, String> {
    match input {
        "off" => return Ok(States::Off),
        "sleep" => return Ok(States::Sleep),
        "hibernate" => return Ok(States::Hibernate),
        "shutdown" => return Ok(States::Shutdown),
        "reboot" => return Ok(States::Reboot),
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
        States::Off => println!("PC is off..."),
        States::Reboot => println!("rebooting..."),
        States::Hibernate => println!("hibernating..."),
        States::Sleep => println!("sleeping mode..."),
        States::Shutdown => println!("shutting down..."),
    }
}

fn main() {
    let user_choice: States = return_state();
    print_state(&user_choice);
}
