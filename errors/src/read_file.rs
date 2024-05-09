use std::fs::File;
use std::path::Path;
use std::io::{Read, Error};

pub fn read_file(file_name: &str) -> Result<String, Error> {
    let file_path: &Path = Path::new(file_name);

    let mut file_content: String = String::new();
    let mut file: File = File::open(file_path)?;

    file.read_to_string(&mut file_content)?; //pega a referecia e declara que o programa pode mutar a variavel file_content e insere o conteudo do arquivo nessa variavel;

    return Ok(file_content);
}
