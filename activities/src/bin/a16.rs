// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>
}

fn main() {
    let students = vec![
        Student { name: "Jayson".to_owned(), locker: Some(132) },
        Student { name: "Mia".to_owned(), locker: None }
    ];

    for student in students {
        match student.locker {
            Some(locker) => println!("{} is assigned to locker {}", student.name, locker),
            None => println!("{} does not have a locker", student.name)
        }
    }
}
