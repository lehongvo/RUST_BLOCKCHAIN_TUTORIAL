/// `pub mod calculator;` is a Rust code snippet that declares a module named `calculator` and makes it
/// public (`pub`). This means that the contents of the `calculator` module can be accessed from outside
/// the current module or crate.
pub mod calculator;
#[cfg(test)]
/// The `pub mod tests { ... }` block in Rust is defining a module named `tests` that is also made
/// public (`pub`). Within this module, there is a test function defined using the `#[test]` attribute,
/// which indicates that this function is a test that should be run by the test harness.
pub mod tests {
    use super::*;
    #[test]
   /// The function `test_calculator` tests the addition operation of a calculator module in Rust.
    fn test_calculator() {
        let value = calculator::try_add(10, 20);
        let mut value_add = 0;
        match value {
            Some(value) => value_add = value,
            None => println!("Invalid value")
        }
        assert_eq!(value_add, 30, "Invalid value");
    }
} 