// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }
}

fn print_message(state: PowerState) {
    use PowerState::*;

    match state {
        Off => println!("Powering off..."),
        Sleep => println!("Going to sleep..."),
        Reboot => println!("Rebooting..."),
        Shutdown => println!("Shutting down in 30 sec.."),
        Hibernate => println!("Hibernating.."),
    }
}

fn main() {
    let mut power_option = String::new();

    println!("What would you like to do?");
    println!("Optoins are...\nOff\nSleep\nReboot\nShutdown\nHibernate");
    io::stdin()
        .read_line(&mut power_option)
        .expect("Failed to read line");

    match PowerState::new(&power_option) {
        Some(state) => print_message(state),
        None => println!("invalid power state")
    }
}
