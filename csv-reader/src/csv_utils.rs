use csv;
use std::error::Error;

pub fn read_csv_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut records_count = 0;

    for result in reader.records() {
        let record = result.expect("Erro to read record");
        println!("{:?}", record);
        records_count += 1;
    }
 
    Ok(records_count)
}