pub fn reverse_array<T>(array: &mut [T]) -> &mut [T] {
  let mut start = 0;
  let mut end = array.len() - 1;

  while start < end {
    array.swap(start, end);

    start += 1;
    end -= 1;
  }

  return array;
}