mod ternary;

fn main() {
    let test = ternary::ternary_fn(10 > 0);

    println!("Resultado ternario: {}", test);
}
