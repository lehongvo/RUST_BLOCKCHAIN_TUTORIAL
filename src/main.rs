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

fn return_stg(mut s: String) -> String {
    s = s + "....OMG";
    return s;
}

fn main() {
    let input_string = String::from("Initial String");
    let current_string = String::from(return_stg(input_string));
    println!("current_string: {}", current_string);
}