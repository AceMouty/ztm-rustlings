// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    fav_color: String
}

fn main() {
    let people = vec![
        Person{ 
            name: "Billy".to_owned(),
            age: 32,
            fav_color: String::from("Red")
        },
        Person{
            name: "Sally".to_owned(),
            age: 21, fav_color:
            String::from("Blue")
        },
        Person{
            name: "Jane".to_owned(),
            age: 1,
            fav_color: String::from("Orange")
        }
    ];

    for person in people {
        if person.age > 10 {
            continue;
        }
        print_name(&person.name);
        print_color(&person.fav_color);
    }
}

fn print_name(name: &str) {
    println!("Name is: {}", name);
}

fn print_color(color: &str) {
    println!("Fav color is: {}", color);
}