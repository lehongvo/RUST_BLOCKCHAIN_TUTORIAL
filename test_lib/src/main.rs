pub mod calculator;

fn main() {
    let value = calculator::try_add(10, 20);
    let mut value_add = 0;
    match value {
        Some(value) => value_add = value,
        None => println!("Invalid value")
    }
    println!("Value: {}", value_add);
}