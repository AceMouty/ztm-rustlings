// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    fn amount(&self) -> u64;
}

struct Carpet(u64); // Tuple struct
impl Material for Carpet {
    fn amount(&self) -> u64 {
        self.0 * 10 // self.0 -> access first item in tuple struct
    }
}

struct Tile(u64);
impl Material for Tile {
    fn amount(&self) -> u64 {
        self.0 * 15
    }
}

struct Wood(u64);
impl Material for Wood {
    fn amount(&self) -> u64 {
        self.0 * 20
    }
}

fn calculate_total(materials: &Vec<Box<dyn Material>>) -> u64 {
    materials.iter().map( | item | item.amount()).sum()
}

fn main() {
    let sqr_ft = 3;
    let carpet = Box::new(Carpet(sqr_ft));
    let tile = Box::new(Tile(sqr_ft));
    let wood = Box::new(Wood(sqr_ft));

    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    let total_cost = calculate_total(&materials);
    println!("Total cost of materials is: {}", total_cost);
}
