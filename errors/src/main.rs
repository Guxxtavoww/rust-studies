// mod panic_function;
mod read_file;

fn main() {
    // panic_function::panic_catch();

    match read_file::read_file("text.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
