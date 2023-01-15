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
use std::collections::HashMap;

fn main() {
    let mut user_selection: String;
    let mut bills: BillList = BillList::new();

    loop {
        menu::print_main_menu();
        user_selection = match get_input() {
            Some(buff) => buff,
            None =>  {
                TerminalManager::clear_screen(0);
                continue;
            }
        };

        let menu_option = match MenuOption::new(&user_selection) {
            Some(val) => val,
            None => {
                println!("Unsupported selection provided");
                break;
            }
        };

        match menu_option {
            MenuOption::AddBill => {
                let new_bill = match menu::create_bill() {
                    Some(val) => val,
                    None => continue
                }; 

                bills.add(new_bill);
                println!("bill added!");
                TerminalManager::clear_screen(3);
            },
            MenuOption::ViewBills => {
                TerminalManager::clear_screen(0);

                if bills.length() == 0 {
                    println!("No bills to display");

                    TerminalManager::clear_screen(2);
                    continue;
                }

                menu::view_bills(bills.get_all()); 

                println!("Press any key to return...");
                io::stdin().read_line(&mut String::new()).unwrap();

                TerminalManager::clear_screen(0);
            },
            MenuOption::RemoveBill => {
                TerminalManager::clear_screen(0);
                menu::remove_bill(&mut bills);
                TerminalManager::clear_screen(2);
            },
            MenuOption::Quit => break,
            _ => {
                println!("Not yet implemented");
                return;
            }
        }
    }

    println!("exiting...");
}


fn get_input() -> Option<String> {

    let mut buff = String::new();
    while io::stdin().read_line(&mut buff).is_err() {
        println!("Please re-enter data");
    }

    let input = buff.trim().to_owned();
    if &input == "" {
        return None;
    }

    Some(input)
}

fn get_bill_amount() -> Option<f64> {
    loop {
        let amount = match get_input() {
            Some(val) => val,
            None => return None
        };

        if &amount == "" {
            return None;
        }

        let parsed_amount: Result<f64, _> = amount.parse();

        match parsed_amount {
            Ok(val) => return Some(val),
            Err(_) => println!("Please enter a number")
        }
    }
}

mod menu {
    use crate::{get_input, get_bill_amount, Bill, BillList};

    pub(crate) fn print_main_menu() {
        println!("== Manage Bills ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("q: quit");
        println!();
    }

    pub(crate) fn create_bill() -> Option<Bill> {

        println!("Bill Name:");
        let name = match get_input() {
            Some(val) => val,
            None => return None
        };

        println!("Bill Amount:");
        let amount = match get_bill_amount() {
            Some(val) => val,
            None => return None
        };

        Some(Bill::new(name, amount))
    }

    pub(crate) fn view_bills(bills_vec: Vec<&Bill>) {
        for bill in bills_vec {
            println!("Name: {} | Amount: {}", bill.name, bill.amount);
        }
    }

    pub(crate) fn remove_bill(bills: &mut BillList) {
        for bill in bills.get_all() {
            println!("Name: {}", bill.name);
        }
        println!("Enter bill name to remove:");

        let bill_name = match get_input() {
            Some(val) => val,
            None => return
        };

        if bills.remove(&bill_name) {
            println!("{} removed", bill_name);
            return;
        }

        println!("{} not found",bill_name);
    }
}

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64
}

impl Bill {
    fn new(name: String, amount: f64) -> Bill {
        Bill { name, amount }
    }
}

struct TerminalManager;
impl TerminalManager {
    fn clear_screen(sec_duration: u64) {

        let sleep_time = Duration::from_secs(sec_duration);
        thread::sleep(sleep_time);
        std::process::Command::new("clear").status().unwrap();
    }
}

struct BillList {
    bills: HashMap<String, Bill>
}

impl BillList {
    fn new() -> Self {
        Self{bills: HashMap::new()}
    }

    fn add(&mut self, new_bill: Bill) {
        self.bills.insert(new_bill.name.to_string(), new_bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.bills.values().collect()
    }

    fn length(&self) -> usize {
        self.bills.len()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.bills.remove(name).is_some()
    }
}

enum MenuOption {
    AddBill,
    ViewBills,
    RemoveBill,
    Quit
}

impl MenuOption {
    fn new(selection: &String) -> Option<MenuOption> {
        
        match selection.as_str() {
           "1" => Some(MenuOption::AddBill),
           "2" => Some(MenuOption::ViewBills), 
           "3" => Some(MenuOption::RemoveBill), 
           "q" => Some(MenuOption::Quit),
            _ => None
        }
    }
}