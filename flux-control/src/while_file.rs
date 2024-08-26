pub fn while_fn(loop_limit: i32) {
    let mut counter = 0;

    while counter <= loop_limit {
        println!("While counter: {}", counter);

        counter += 1;
    }
}