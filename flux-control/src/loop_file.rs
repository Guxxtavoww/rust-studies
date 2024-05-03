pub fn loop_fn() -> () {
  let mut counter: i32 = 0;

  loop {
    println!("Counter: {}", counter);

    counter += 1;

    if counter > 10 {
      break;
    }
  }
}