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

// use std::io;
// fn main() {
//     println!{"Guess the number!"};
//     println!("Please input your guess.");
//     let mut you_number: String = String::new();
//     io::stdin()
//         .read_line(&mut you_number)
//         .expect("Error reading");
//     println!("Please input your guess.");
//     println!("You guessed: {}", you_number)
// }

// use rand::Rng;
// use std::io;
// fn main() {
//     println!("PLease guess my number!!!\n");
//     println!("PLease input your number from(0 to u8)");

//     // Input number
//     let mut system_input: String = String::new();
//     io::stdin()
//         .read_line(&mut system_input)
//         .expect("Error reading");

//     // Read number
//     let system_value: u8 = match system_input.trim().parse() {
//         Ok(value) => value,
//         Err(_) => {
//             println!("Đầu vào không hợp lệ. Vui lòng nhập kiểu dữ u8");
//             return;
//         }
//     };

//     // Ramdon number
//     let mut count_time: u128 = 0;
//     for n in 0..=1000000 {
//         let mut rng = rand::thread_rng();
//         let random_number: u8 = rng.gen_range(0..127);
//         println!("Random number is: {}", random_number);
//         if (random_number == system_value) {
//             count_time += 1;
//         }
//     }
//     println!("You win {} times", count_time);
// }

// use std::io;
// fn main() {
//     println!("Guess your number!!");
//     println!("Please input you number");
//     let mut string_value = String::new();
//     io::stdin()
//         .read_line(&mut string_value)
//         .expect("Invalid input");
//     print!("You data is: {}", string_value)
// }

// use std::io;
// fn read_number(prompt: &str) -> u128 {
//     println!("Please enter {}", prompt);
//     let mut value: String = String::new();
//     io::stdin().read_line(&mut value).expect("Error reading");
//     let value_input: u128 = match value.trim().parse() {
//         Ok(value) => value,
//         Err(_) => {
//             println!("Invalis number");
//             std::process::exit(1)
//         }
//     };
//     return value_input;
// }

// fn main() {
//     let value_one: u128 = read_number("Value 1");
//     let value_two: u128 = read_number("Value 2");
//     let total_value: u128 = value_one + value_two;
//     println!("total_value is: {}", total_value);
// }

// use rand::Rng;
// use std::{cmp::Ordering, io};
// fn main() {;
//     loop {
//         println!("===========>PLease input your value<===========");
//         let max_size: u8 = 100;
//         let mut rng = rand::thread_rng();
//         let secret_number: u8 = rng.gen_range(1..=max_size);

//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("Error reading");
//         let guess: u8 = match guess.trim().parse() {
//             Ok(value) => {
//                 if (value > max_size) {
//                     continue;
//                 } else {
//                     value
//                 }
//             }
//             Err(_) => {
//                 println!("Invalid guess. Please input correct number");
//                 continue;
//             }
//         };
//         println!("You guessed: {}", guess);
//         println!("You secrer number: {}", secret_number);
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Less then secret number"),
//             Ordering::Greater => println!("Great then secret number"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

// fn main() {
//     let mut x: i32= 5;
//     println!("The value of x is: {}", x);
//     x = x + 1;
//     println!("The value of x after chain is: {}", x);
// }

// fn main() {
//     let x: i32 = 100;
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}")
// }

//Shadowing
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     println!("The value of x is {}", x);
//     {
//         let x = x + 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x under scope is {}", x);
// }

// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("Error: {}", spaces)
// }
// fn main() {
//     let number_string: f64 = "7232232.12".parse().expect("Not a number");
//     println!("number_string: {}", number_string);
// }

// fn main () {
//     let mut bool_data: bool = false;
//     bool_data = true;
//     println!("bool_data {}", bool_data);
// }

// fn main() {
//     let char_value: char = 'A';
//     let mut string_value: String = String::from("hello world");
//     string_value = string_value + &char_value.to_string();
//     println!("string_value {}", string_value);
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }
// Tuple data type in rust
// fn main () {
//     println!("HELLO");
//     let myData: (i32, f64) = (32, 12.12);
//     let data1 = myData.0;
//     let data2 = myData.1;
//     println!("Data 1 : {}, Data 2 : {}", data1, data2);
// }

// fn main () {
//     let mut x: u8;
//     let mut y: f64;
//     let my_tuple: (u8, u8) = (12, 123);
//     (x, y) = (my_tuple.0, my_tuple.1 as f64);
//     println!("Value x is {}, Value y is {}", x, y)
// }

// fn return_function(value1: u128, value2: u128) -> (u8, bool) {
//     let mut is_bool: bool = false;
//     let current_value: u8 = 0;
//     if value1 > value2 {
//         is_bool = true;
//         let current_value: u8 = (value1 - value2) as u8;
//         return(current_value, is_bool)
//     }
//     return(value1 as u8, is_bool)

// }

// fn main () {
//     let val1: u128 = 10;
//     let val2: u128 = 9;
//     println!("The value is {:?}", return_function(val1, val2));
// }

// fn main() {
//     let tuple_value: (i32, f64, u8) = (500, 6.4, 1);
//     println!("tuple_value, {:?}", tuple_value);
// }

// fn main () {
//     let tup: (i32, i32, bool, bool, f64, &str) = (200, 12, true, false, 3.444, "HELLO");
//     let (x, y, z, g, h, e) = tup;
//     println!("Value is {}", e);
// }

// fn main () {
//     let x: (i32, f64, u8) = (400, 8.1, 1);
//     let five_hundred = x.0;
//     let value_point_one = x.1;
//     let one = x.2;
//     println!("data: {}", value_point_one);
// }

// fn main() {
//     let months: [&str; 12]= ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
//     let tup: (i8, [&str; 12]) = (1, months);
//     println!("Value is {:?}", tup);
// }

// fn main() {
//     let a: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9 ,120];
//     print!("New array is {:?}", a);
// }
// fn main() {
//     let a = [3,4];
//     println!("New array using default type is {:?}", a);
// }

// fn main() {
//     let a = [1,2,3,4,5,6,7,8];
//     let first_value = a[3];
//     println!("first_value {}", first_value);
// }

// use std::io;
// fn main() {
//     let a = [1, 2, 3, 4, 5, 6, 7, 8];
//     let length_array = a.len();

//     let mut value_input: String = String::new();

//     println!("PLease in put you number");
//     io::stdin()
//         .read_line(&mut value_input)
//         .expect("Error reading");
//     let index_value: usize = match value_input.trim().parse() {
//         Ok(value) => {
//             if value > length_array {
//                 println!("Please input from 0 to {length_array}");
//                 return;
//             }
//             value // Use 'value' instead of 'return value'
//         }
//         Err(_) => {
//             println!("Error parsing data");
//             std::process::exit(1);
//         }
//     };

//     println!("Your number is {}", a[index_value]);
// }

// fn add_number(value1: u8, value2: u8) -> u8 {
//     let value: u8 = value1 + value2;
//     return value;
// }
// fn main() {
//     let value = add_number(3, 4);
//     println!("data: {}", value);
// }

// fn great_name(name: &str) -> &str {
//     let current_str: &str = &(name.to_string() + "is best name");
//     return current_str;
// }

// fn great_name(name: &str) -> String {
//     let current_str: String = name.to_string() + "is the best name";
//     return current_str;
// }

// fn main() {
//     let name: String= String::from("vali");
//     let string_value: String= great_name(&name);
//     println!("Value is: {}", string_value)
// }

// fn another_function() {
//     println!("Another function");
// }

// fn main() {
//     println!("Hello world");
//     another_function();
// }

// fn another_function(value: u8) {
//     println!("Another function says: {}", value);
// }
// fn main() {
//     another_function(4);
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The print_labeled_measurement is {value}{unit_label}");
// }
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn main() {
//     let y = {
//         let x: i32 = 10;
//         x + 1
//     };
//     println!("Value of y is {y}")
// }

// fn return_data(value: i128) -> i128 {
//     let max_value: i8 = i8::MAX;
//     let min_value: i8 = i8::MIN;
//     if value < 10 || value > 1000 {
//         println!("Invalis valus {value}, please input value from {min_value} to {max_value} ");
//         return 0;
//     }
//     let value: i128 = value % 100;
//     return value;
// }

// fn main() {
//     let value: i128 = return_data(1200);
//     println!("Value is {value}");
// }

// fn plus(x: i128, y: i128) -> i128 {
//     let total_value: i128 = x + y;
//     return total_value;
// }

// fn main() {
//     let value: i128 = plus(10, 20);
//     println!(" Data is {value}");
// }

// hello World

// fn main() {
//     // I'm feeling lucky number
//     let lucky_number = 7;

// }

// fn main() {
//     let number = 3;
//     if(number < 5) {
//         println!("Condition was true");
//     } else {
//         println!("Condition was false")
//     }
// }

// fn main() {
//     let number = 0;
//     if number != 0 {
//         println!("Number was something other than zero");
//     } else {
//         println!("Great number");
//     }
// }

// fn main() {
//     let number = 5;
//     if number % 4 == 0 {
//         println!("Number is divisible by 4");
//     } else {
//         if number % 3 == 0 {
//             println!("Number is divisible by 3");
//         } else {
//             if number % 2 == 0 {
//                 println!("Number is divisible by 2");
//             } else {
//                 println!("number is not divisible by 4, 3, or 2");
//             }
//         }
//     }
// }

// fn main() {
//     let number = 4;
//     match number {
//         n if n % 4 == 0 => println!("Number is divisible by 4 {}", n),
//         m if m % 3 == 0 => println!("Number is divisible by 3"),
//         m if m % 2 == 0 => println!("Number is divisible by 2"),
//         _ => println!("Number is not divisible by 4, 3, or 2"),
//     }
// }

// fn main() {
//     let condition = false;
//     let number = if condition { 1 } else { 0 };
//     println!("Number: {}", number);
// }

// fn main() {
//     let condition = false;
//     let number = if condition { 1 } else { "322".parse().expect("Not a number") };
//     println!("Number: {}", number);
// }

// fn main() {
//     let mut string_data: String = String::new();
//     let mut counter: i128 = 0;
//     loop {
//         let secret_number = rand::random::<i32>();
//         counter += 1;
//         if(secret_number == 10) {
//             println!("---Number: {}", secret_number);
//             println!("---counter: {}", counter);
//             break;
//         }
//     }
// }

// use tokio::time::{sleep, Duration};

// async fn async_task() {
//     let mut count: u128 = 0;

//     'count_up: loop {
//         println!("------{}", count);

//         if count == 100_000_000_000 {
//             break 'count_up;
//         }

//         count += 1;

//         // Asynchronously sleep for 1 second
//         sleep(Duration::from_secs(2)).await;
//     }
// }

// #[tokio::main]
// async fn main() {
//     async_task().await;
// }

// fn main() {
//     let mut number = 1000;
//     while number > -11111110 {
//         println!("Value number is: {}", number);
//         number -= 1;
//     }
//     print!("DONE");
// }

// fn main() {
//     let array = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while index <= array.len() {
//         println!("Value is {}", array[index]);
//         index += 1;
//     }
// ;}

// fn main() {
//     let array = [10, 20, 30, 40, 50];
//     for element in array.iter() {
//         println!("Element is: {}", element);
//     }
// }

// fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
//     let data: f64 = (fahrenheit - 32 as f64) * 5 as f64 / 9 as f64;
//     return data;
// }

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     let data: f64 = celsius * 9.0 / 5.0  + 32.0;
//     return data;
// }

// fn main() {
//     println!("Data is: {}", fahrenheit_to_celsius(1000.0));
//     println!("Data is: {}", celsius_to_fahrenheit(537.0));
// }

// fn fibonacci_recursive(number: u128) -> u128 {
//     if number == 0 {
//         return 0;
//     };

//     if number == 1 {
//         return 1;
//     }

//     println!("Hello {}", number);

//     return fibonacci_recursive(number - 1) + fibonacci_recursive(number - 2);
// }

// fn main() {
//     let result = fibonacci_recursive(10);
//     println!("The Fibonacci number is: {}", result);
// }
fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!("On the first day of Christmas, my true love gave to me:");
    for day in 1..=12 {
        println!("{}{}", day, if day == 1 { "st" } else { "th" });

        for gift_day in (1..=day).rev() {
            println!("{}{}", if gift_day == 1 && day != 1 { "And " } else { "" }, gifts[gift_day - 1]);
        }

        println!(); // Empty line between days
    }
}
