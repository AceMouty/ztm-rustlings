// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perim {
    fn calc_perimeter(&self) -> u64;
}

struct Square{
    l: u64
}

impl Perim for Square {
    fn calc_perimeter(&self) -> u64 {
        self.l * 4
    }
}

struct Triangle{
    a: u64,
    b: u64,
    c: u64
}

impl Perim for Triangle {
    fn calc_perimeter(&self) -> u64 {
        self.a + self.b + self.c
    }
}

fn print_perimeter(shape: impl Perim){
    println!("The perimeter is: {}", shape.calc_perimeter());
}

fn main() {
    print_perimeter(Square { l: 3 });
    print_perimeter(Triangle{ a: 4, b: 12, c: 30});
}
