// fn print_str(s: &str) {
//     println!("print: {}", s);
// }

// fn main() {
//     let my_str: &str = "Hello, Rust!";
//     print_str(my_str);
// }

// fn print_stg(s: &str) {
//     println!("print: {}", s);
// }

// fn main() {
//     let my_str: &str = "Hello, Rust";
//     let new_sth = my_str.to_owned() + "....No";
//     println!("new_sth: {}", new_sth)
// }

// fn return_stg(mut s: String) -> String {
//     s = s + "....OMG";
//     return s;
// }

// fn main() {
//     let input_string = String::from("Initial String");
//     let current_string = String::from(return_stg(input_string));
//     println!("current_string: {}", current_string);
// }

// fn main() {
//     let my_str: &str = "Hello, Rust";
//     let current_str: &str =  &(my_str.to_owned() + "... Ok Pro");
//     println!("current_str: {}", current_str)
// }

// fn main() {
//     let full_str: &str = "Hello, Rust!";
//     let part_str: &str = &full_str[5..8];
//     println!("part_str: {}", part_str)
// }

// fn main() {
//     let mut my_string: String = String::from("Hello, Rust!");
//     my_string.push_str("Hello, Rust!");
//     println!("my_string, {}", my_string);
// }

// fn main() {
//     let my_string: String = String::from("Hello, Rust!");
//     let my_data: &str = "Hello, Rust!";
//     let length_value: usize = my_string.len();
//     let length_my_data: usize = my_data.len();
//     println!("Value length: {}", length_value);
//     println!("my_data length: {}", length_my_data);
// }

// fn main() {
//     let is_empty_str: &str = "";
//     let is_empty_string: String = String::from("Hello, Rust!");
//     let is_right_str: bool = is_empty_str.is_empty();
//     let is_right_strong: bool = is_empty_string.is_empty();
//     println!("is_right_str: {}", is_right_str);
//     println!("is_right_str: {}", is_right_strong);
// }

// fn main() {
//     let is_contain_str: &str =  "Hello, world";
//     let is_contain_strong: String = String::from("Ok pro");
//     if 
//       is_contain_str.to_uppercase().contains(&("OK").to_uppercase()) && 
//       is_contain_strong.to_uppercase().contains(&("HELLO").to_uppercase())
//     {
//         println!("Yes {} is correctly contained", is_contain_str);
//     } else {
//         println!("No is not correctly contained");
//     }
// }

// fn main () {
//     let current_float: String = String::from("9.9999999999999911");
//     let value_float: f64 = current_float.parse().expect("Not a number float");
//     println!("value_float: {}", value_float)
// }

// fn main() {
//     let mut string_new: String = String::new();
//     string_new.push('O');
//     string_new.push_str("LE HONG VO");
//     println!("string_new: {}", string_new);
// }

use std::io;
fn main() {
    println!{"Guess the number!"};
    println!("Please input your guess.");
    let mut you_number: String = String::new();
    io::stdin()
        .read_line(&mut you_number)
        .expect("Error reading");
    println!("Please input your guess.");
    println!("You guessed: {}", you_number)
}
