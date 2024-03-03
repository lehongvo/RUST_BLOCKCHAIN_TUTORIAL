use safemath_dev::math::{try_add};
fn main() {
    let value = try_add(10, 12).unwrap();
    println!("value: {}", value);
}
