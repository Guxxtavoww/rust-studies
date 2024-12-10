use crate::practice::PraticeTrait;
use crate::person::handle_person;

mod person;
mod practice;

trait GeometricForm {
    // cria uma interface de tipagem para ser usado na implementacao de structs
    fn calculate_area(&self) -> u32;

    fn new(width: u32, height: u32) -> Self;
}

struct Square {
    height: u32,
    width: u32,
}

impl GeometricForm for Square {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self { height, width }
    }
}

fn main() {
    let practice = practice::PraticeStruct::new(String::from("Oi"));
    let square = Square::new(10, 10);

    let area = square.calculate_area();

    println!("Area: {}, Pratice: {}, Name: {}", area, practice.description, handle_person(String::from("Fodase"), String::from("Fodase")));
}
