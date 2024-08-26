pub fn panic_function(value: i32) -> i32 {
    if value == 0 {
        panic!("O valor não pode ser zero"); // === throw new Error()
    }

    return value;
}

pub fn panic_catch() {
    let result = std::panic::catch_unwind(|| {
        let value = panic_function(10);

        return Ok::<i32, &str>(value);
    });

    match result {
        Ok(val) => println!("{}", val.unwrap()),
        Err(_) => println!("ppppppp")
    }
}
