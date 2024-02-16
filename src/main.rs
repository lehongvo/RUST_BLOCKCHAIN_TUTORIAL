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

/// The code removes the character at index 1 from the string "Hello, Rust!" and assigns the modified
/// string to a new variable called new_string.
fn main() {
    let mut my_string: String = String::from("Hello, Rust!");
    my_string.remove(1);
    my_string.insert(1, 'E');
    println!("my_string, {}", my_string);
}