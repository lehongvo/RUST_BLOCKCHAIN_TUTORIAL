fn main() {
    let string_value: &str = "Hello, world!";
    let mut binding = string_value.to_owned() + "world!";
    let bindingUp: &str = &binding.to_uppercase();
    let bindingLow: &str = &binding.to_lowercase();
    println!("bindingUp: {}", bindingUp);
    println!("bindingLow: {}", bindingLow);
}