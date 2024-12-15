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

pub fn sort_array<T>(array: &mut [T], sorting_type: &str) where T: Ord, {
  let parsed_sorting_type = sorting_type.to_lowercase();

  if parsed_sorting_type != "asc" && parsed_sorting_type != "desc" { panic!("Invalid sorting type!") }

  if parsed_sorting_type == "asc" {
    array.sort()
  } else {
    array.sort_by(|a, b| b.cmp(a))
  }
}