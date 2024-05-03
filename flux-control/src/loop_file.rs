pub fn loop_fn() -> () {
  let mut counter: i32 = 0;

  loop {
    println!("Loop Counter: {}", counter);

    counter += 1;

    if counter > 10 {
      break;
    }
  }
}