// mod panic_function;
mod read_file;
mod divide;

fn main() {
    // panic_function::panic_catch();

    match read_file::read_file("text.txt") {
        Ok(content) => println!("File content: \n {}", content),
        Err(err) => eprintln!("Erro: {}", err),
    }

    match divide::divide_fn(10.4, 2.0) {
        Some(value) => println!("Result: {}", value),
        None => eprintln!("Divisor não pode ser 0")
    }
}
