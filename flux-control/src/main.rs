mod loop_file;
mod ternary;
mod while_file;

fn main() {
    let test = ternary::ternary_fn(10 > 0);

    println!("Resultado ternario: {}", test);

    // loop_file::loop_fn();
    while_file::while_fn(100);
}
