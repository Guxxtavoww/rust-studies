pub fn divide_fn(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
       return None;
    }

    Some(dividend / divisor)
}