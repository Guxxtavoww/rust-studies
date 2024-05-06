struct Person {
    name: String,
    age: i8,
    height: f32,
}

fn main() {
    let me = Person {
        age: 20,
        height: 1.70,
        name: String::from("Gustavo"),
    };

    println!("Nome: {}, Idade: {}, Altura: {}", me.name, me.age, me.height);
}
