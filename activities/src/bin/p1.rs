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
    let mut user_selection: String;
    let mut bills: BillList = BillList::new();

    loop {
        print_menu();
        match get_input() {
            Ok(buff) => user_selection = buff,
            Err(err) =>  {
                println!("{}", err);
                break;
            }
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

                println!("Name | Amount");
                bills.print();

                println!("Press any key to return...");
                io::stdin().read_line(&mut String::new()).unwrap();

                TerminalManager::clear_screen(0);
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

fn print_menu() {

    println!("== Manage Bills ==");
    println!("1. Add Bill");
    println!("2. View Bills");
    println!("q: quit");

    // empty spacer from menu
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

struct TerminalManager;
impl TerminalManager {
    fn clear_screen(sec_duration: u64) {

        let sleep_time = Duration::from_secs(sec_duration);
        thread::sleep(sleep_time);
        std::process::Command::new("clear").status().unwrap();
    }
}

struct BillList {
    bills: Vec<Bill>
}

impl BillList {
    fn new() -> Self {
        Self{bills: Vec::new()}
    }

    fn add(&mut self, new_bill: Bill) {
        self.bills.push(new_bill);
    }

    fn print(&self) {
        self.bills.iter().for_each(| bill | println!("{} {}", bill.name, bill.amount));
    }

    fn length(&self) -> usize {
        self.bills.len()
    }
}

enum MenuOption {
    AddBill,
    ViewBills,
    Quit
}

impl MenuOption {
    fn new(selection: &String) -> Option<MenuOption> {
        
        match selection.as_str() {
           "1" => Some(MenuOption::AddBill),
           "2" => Some(MenuOption::ViewBills), 
           "q" => Some(MenuOption::Quit),
            _ => None
        }
    }
}