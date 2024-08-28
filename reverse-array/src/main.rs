mod array_utils;

use array_utils::reverse_array;

fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    reverse_array(&mut arr);

    println!("{:?}", arr);
}
