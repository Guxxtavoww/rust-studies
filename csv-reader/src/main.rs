mod csv_utils;
use csv_utils::read_csv_file;

fn main() {
    match read_csv_file("./file.csv") {
        Ok(records_count) => println!("Sucesso: {} linhas lidas", records_count),
        Err(e) => panic!("{}", e)
    }
}
