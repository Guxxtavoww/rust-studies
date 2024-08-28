mod array_utils;

use array_utils::{ reverse_array, sort_array };

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 3];

    reverse_array(&mut arr);

    println!("{:?}", arr);

    sort_array(&mut arr, "desc");

    println!("Sorted: {:?}", arr)
}
