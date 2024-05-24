mod utils;

fn main() {
    match utils::file_reader::file_reader("test.txt") {
        Ok(file_content) => println!("{}", file_content),
        Err(err) => println!("{}", err),
    }

    println!("{}", utils::get_current_date::get_current_date());
}
