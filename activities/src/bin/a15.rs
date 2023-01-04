// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Standard(i32),
    Backstage(i32, String),
    Vip(i32, String)
}


fn main() {
    let tickets = vec![
        Ticket::Standard(18),
        Ticket::Backstage(200, String::from("Jane")),
        Ticket::Vip(1000, String::from("Nick")),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price)=> println!("Standard ticket cost {}", price),
            Ticket::Backstage(price, holder_name) => println!("Backstage ticket holder {} price {}", holder_name, price),
            Ticket::Vip(price, holder_name) => println!("Vip ticket holder {} price {}", holder_name, price)
        }
    }
}
