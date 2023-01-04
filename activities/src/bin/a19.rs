// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;
fn main() {
    let mut furniture_map = HashMap::new();

    furniture_map.insert("Chairs".to_owned(), 5);
    furniture_map.insert("Beds".to_owned(), 3);
    furniture_map.insert("Tables".to_owned(), 2);
    furniture_map.insert("Couches".to_owned(), 0);

    let mut items_in_stock = 0;

    for(item, qty) in furniture_map.iter() {
        match qty {
            0 => println!("{}: Out of Stock", item),
            _ => {
                println!("{}: {} in stock", item, qty);
                items_in_stock += 1
            } 
        }
    }

    println!("Total items in stock: {}", items_in_stock);
    
}
