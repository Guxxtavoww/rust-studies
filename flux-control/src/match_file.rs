pub fn match_fn(current_station: &str) {
    match current_station {
        // Equivalente ao switch
        "primaveira" => {
            println!("Uau que dlc");
        }
        _ => {
            // condicao default
            println!("Estacao provavelmente invalida");
        }
    }
}
