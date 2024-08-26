mod loop_file;
mod ternary;
mod while_file;
mod for_file;
mod match_file;

fn main() {
    let test = ternary::ternary_fn(10 > 0);

    println!("Resultado ternario: {}", test);

    loop_file::loop_fn();
    while_file::while_fn(5);
    for_file::for_fn();
    match_file::match_fn("fodase")
}
