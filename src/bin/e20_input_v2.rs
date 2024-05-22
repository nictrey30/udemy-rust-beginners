// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
// * Off, * Sleep, * Reboot, * Shutdown, * Hibernate
// * If the user enters one of the keywords, a message should be printed to the console indicating which action will be taken
// * Example: If the user types in "shutdown" a message should display such as "shutting down"
// * If the keyword entered does not exist, an appropriate error message should be displayed
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
// * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type Reboot, reboot, REBOOT, etc.)

use std::io;

// * Use an enum to store the possible power states
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

// * Use a function with a match expression to print out the power messages
impl PowerState {
    // the reason we are returning an option is that the user can input something that is not a power state
    fn new(state: &str) -> Option<Self> {
        let state = state.to_lowercase();
        match state.trim() {
            "off" => Some(Self::Off),
            "sleep" => Some(Self::Sleep),
            "reboot" => Some(Self::Reboot),
            "shutdown" => Some(Self::Shutdown),
            "hibernate" => Some(Self::Hibernate),
            _ => None,
        }
    }
}

// * Use a function with a match expression to print out the power messages
fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("turning off"),
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating"),
    }
}
fn main() {}
