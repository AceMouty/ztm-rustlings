// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    name: String,
    age: i32
}
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
impl Adult {
    fn from(name: &str, age: i32) -> Result<Self, String> {
        if age < 21 {
           return Err("Age must be 21 or older".to_string())
        }

        return Ok(Self {age, name: name.to_string()})
    }
}
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message


fn main() {
    let a1: Result<Adult, String> = Adult::from("Billy", 23);
    match a1 {
        Ok(adult) => println!("Adult name: {}, age: {}", adult.name, adult.age),
        Err(e) => println!("Err: {}", e)
    }

    let a2 = Adult::from("Mandy", 18);
    match a2 {
        Ok(adult) => println!("Created adult: {:?}", adult),
        Err(message) => println!("{}", message)
    }
    
}
