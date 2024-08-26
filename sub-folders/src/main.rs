mod utils;

use utils::{file_reader, get_current_date};

fn main() {
    match file_reader::file_reader("test.txt") {
        Ok(file_content) => println!("{}", file_content),
        Err(err) => println!("{}", err),
    }

    println!("{}", get_current_date::get_current_date());
}
