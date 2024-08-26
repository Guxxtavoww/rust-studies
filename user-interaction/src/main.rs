use std::io::stdin;

fn main() {
    let mut user_input = String::new();

    match stdin().read_line(&mut user_input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("Texto inserido: {user_input}");
        }
        Err(error) => println!("error: {error}"),
    }
}
