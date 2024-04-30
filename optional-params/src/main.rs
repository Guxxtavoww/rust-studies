fn greet(name: &str, custom_greet: Option<&str>) -> () {
  match custom_greet { // Switch case
    Some(s) => println!("{}, {}", s, name),
    None => println!("Salve {}!", name)
  }
}

fn main() {
  greet("Jailson Mendes", Some("Kd o suco de laranja"));
  greet("Paulo Guina", None);
}
