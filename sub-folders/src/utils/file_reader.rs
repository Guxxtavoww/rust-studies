use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

pub fn file_reader(whole_file_name: &str) -> Result<String, Error> {
    let file_path = Path::new(whole_file_name);

    let mut file_content = String::new();
    let mut file: File = File::open(file_path)?;

    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}
