pub struct Person {
    pub(crate) name: String,
    pub(crate) age: i8,
    pub(crate) height: f32,
}

impl Person {
    pub(crate) fn print_values(&self) {
        println!(
            "Nome: {}, Idade: {}, Altura: {}",
            self.name, self.age, self.height
        );
    }
}