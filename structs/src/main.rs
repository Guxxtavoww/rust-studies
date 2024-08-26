mod person;
mod square;

fn main() {
    let me = person::Person {
        age: 20,
        height: 1.70,
        name: String::from("Gustavo"),
    };

    let square = square::Square::new_square(3.1, 3.21);

    let area = square.calculate_area();
    let perimeter = square.calculate_perimeter();
    let is_perfect_square = square.is_perfect_square();

    println!(
        "Area do quadrado: {}, Perimetro: {}, É Perfeito ? {}",
        area, perimeter, is_perfect_square
    );

    me.print_values();
}
