// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//          - Bill struct with name and amount fields
//          - a vec so that we can push values into it
//   - I want to view existing bills.
//          - further evidance of needing a vec so that we can view currently added items
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bills: Vec<Bill> = Vec::new();
    let mut user_selection: String;

    loop {
        print_menu();
        match get_input() {
            Ok(buff) => user_selection = buff,
            Err(err) =>  {
                println!("{}", err);
                break;
            }
        }

        if user_selection == "q" {
            break;
        }

        let menu_result =  MenuOption::new(&user_selection);
        if menu_result.is_none() {
            println!("Unsupported selection provided");
            break;
        }

        let menu_option = menu_result.unwrap();
        match menu_option {
            MenuOption::AddBill => {
                let new_bill = create_new_bill(); 
                bills.push(new_bill);
                println!("bill added!");

                let sleep_sec = Duration::from_secs(3);
                thread::sleep(sleep_sec);
                std::process::Command::new("clear").status().unwrap();
            },
            MenuOption::ViewBills => {
                std::process::Command::new("clear").status().unwrap();
                if bills.len() == 0 {
                    println!("No bills to display");

                    let sleep_sec = Duration::from_secs(2);
                    thread::sleep(sleep_sec);
                    std::process::Command::new("clear").status().unwrap();
                    continue;
                }

                println!("Name | Amount");
                bills
                .iter()
                .for_each(| bill | println!("{} | {}", bill.name, bill.amount));

                // let sleep_sec = Duration::from_secs(3);
                // thread::sleep(sleep_sec);
                println!("Press any key to return...");
                io::stdin().read_line(&mut String::new());
                std::process::Command::new("clear").status().unwrap();
            }
            _ => {
                println!("Not yet implemented");
                return;
            }
        }
    }

    println!("exiting...");
}

fn print_menu() {

    println!("== Manage Bills ==");
    println!("1. Add Bill\n2. View Bills");

    // empty spacer from menu
    println!("q: quit");
    println!();
}

fn get_input() -> io::Result<String> {

        let mut buff = String::new();

        io::stdin().read_line(&mut buff)?;
        Ok(buff.trim().to_owned())
}

fn create_new_bill() -> Bill {
    let mut buff = String::new();

    println!("What is the name of this new bill?");
    io::stdin().read_line(&mut buff).expect("create_new_bill: Unable to read input");
    let name = buff.trim().to_owned();

    buff.clear();
    println!("What is the cost of this bill?");
    io::stdin().read_line(&mut buff).expect("create_new_bill: Unable to read input");
    let cost = buff.trim().parse::<f32>().unwrap();

    Bill::new(name, cost)
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f32
}

impl Bill {
    fn new(name: String, amount: f32) -> Bill {
        Bill { name, amount }
    }
}

// Possible options in the application
enum MenuOption {
    AddBill,
    ViewBills
}

impl MenuOption {
    fn new(selection: &String) -> Option<MenuOption> {
        
        match selection.as_str() {
           "1" => Some(MenuOption::AddBill),
           "2" => Some(MenuOption::ViewBills), 
            _ => None
        }
    }
}