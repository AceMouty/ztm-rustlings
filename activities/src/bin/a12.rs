// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Vanilla,
    Brown
}

impl Color {
    fn print(&self) {
        match self {
            Color::Vanilla => println!("color: Vanilla"),
            Color::Brown => println!("color: Brown")
        }
    }
}

struct BoxDimensons {
    l: f64,
    w: f64,
    h: f64
}

impl BoxDimensons {
    fn print(&self) {
        println!("length: {}", self.l);
        println!("width: {}", self.w);
        println!("height: {}", self.h);
    }
}

struct Box {
    color: Color,
    weight: f64,
    dimensions: BoxDimensons,
}

impl Box {
    fn from(weight: f64, dimensions: BoxDimensons, color: Color) -> Self {
        Self { weight, dimensions, color }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {}", self.weight); 
    }
}

fn main() {
    let dimensions = BoxDimensons { l: 23.0, w: 8.5, h: 5.5 };
    let shipping_box = Box::from(10.5, dimensions, Color::Brown);
    shipping_box.print();
}
