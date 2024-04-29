fn type_of<T>(_: T) -> &'static str {
    return std::any::type_name::<T>(); // retorna o tipo de um argumento como uma referencia estatica de uma string
}

fn main() {
    let int = 10;
    let int_to_float = int as f32;
    let float_to_string: String = int_to_float.to_string();
    let string_to_int = float_to_string.parse::<i32>().expect("Invalid number"); // parse espera um tipo e retorna um objeto que contem um resultado, é possivel usar o método unwrap();

    println!(
        "Int to float: {} \n Float to String: {} \n String to int value: {} \n Type of string to int: {}",
        type_of(int_to_float),
        type_of(float_to_string),
        string_to_int,
        type_of(&string_to_int) // é preciso usar a referencia da variavel; 
    );
}
