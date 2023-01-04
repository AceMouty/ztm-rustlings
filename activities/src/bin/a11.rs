// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quanity: i32
}

fn main() {
    let milk = GroceryItem { id: 2, quanity: 1 };
    print_quanity(&milk);
    print_id(&milk);
}

fn print_quanity(grocery_item: &GroceryItem) {
    println!("{}", grocery_item.quanity);
}

fn print_id(grocery_item: &GroceryItem) {
    println!("{}", grocery_item.id);
}
