use std::io::stdin;

fn main() {
    let mut user_input = String::new();

    stdin().read_line(&mut user_input).ok().expect("Erro ao ler a linha");

    if user_input.is_empty() {
        
    }

    println!("Texto inserido: {}", user_input)
}
