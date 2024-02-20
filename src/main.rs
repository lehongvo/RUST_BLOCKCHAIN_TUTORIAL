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
// fn main() {
//     let gifts = [
//         "A partridge in a pear tree",
//         "Two turtle doves",
//         "Three French hens",
//         "Four calling birds",
//         "Five golden rings",
//         "Six geese a-laying",
//         "Seven swans a-swimming",
//         "Eight maids a-milking",
//         "Nine ladies dancing",
//         "Ten lords a-leaping",
//         "Eleven pipers piping",
//         "Twelve drummers drumming",
//     ];

//     println!("On the first day of Christmas, my true love gave to me:");
//     for day in 1..=12 {
//         println!("{}{}", day, if day == 1 { "st" } else { "th" });

//         for gift_day in (1..=day).rev() {
//             println!("{}{}", if gift_day == 1 && day != 1 { "And " } else { "" }, gifts[gift_day - 1]);
//         }

//         println!(); // Empty line between days
//     }
// }

// fn main() {
//     let s1: String = String::from("Hello");
//     let s2: String = s1.clone();
//     println!("Value is s1{}", s1);
//     println!("Value is s2{}", s2);
// }

// fn print_length(value: &String) {
//     println!("Length: {}", value.len());
// }

// fn main() {
//     let value: String = String::from("sisss");
//     print_length(&value);
//     println!("Value is {value}");
// }

// fn longest<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str where 'b:'a {
//     // Có thể chọn thời gian sống là `'a` hoặc `'b`
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }


// fn longest<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }
// fn main() {
//     let s1 = String::from("HELLO");
//     let result;
//     {
//         let s2 = String::from("HELLO");
//         result = longest(&s1, &s2);
//     }
//     println!("{}", result);
// }

// fn longest<'a>(data1: &'a str, data2: &'a str) -> &'a str {
//     if data1.len() > data2.len() {
//         data1
//     } else {
//         data2
//     }
// }

// fn main() {
//     let str1 = String::from("Hello world1!!!");
//     let str2 = String::from("Hello world2");

//     let result;
//     {
//         let r1 = &str1;
//         let r2 = &str2;
//         result = longest(r1, r2);
//     }
//     println!("result = {}", result);
// }

// fn main () {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("{}, world!", s1);
// }


// fn main () {
//     let mut x = 5;
//     x = 7;
//     println!("{}, world!", x);
// }

// fn give_ownership() -> String {
//     let some_string: String = String::from("hello");
//     return some_string;
// }

// fn take_and_gives_back(c: Stirng) -> String {
//     return   let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);;
// }

// fn takes_ownership(some_string: String) {
//     println!("Values are {some_string}");
// }

// fn make_copy(some_integer: i32) {
//     println!("values are {some_integer}");
// } 

// fn main() {
//     let value_string = String::from("hello");
//     takes_ownership(value_string);
//     println!("Values are {value_string}");
//     // let x = 5;
//     // make_copy(x);
//     // println!("Value is {x}");
// }

// fn give_ownership() -> String {
//     let some_string: String = String::from("hello");
//     return some_string;
// }

// fn take_and_gives_back(value: String) -> String {
//     return value
// }

// fn main() {
//     let s1 = give_ownership();
//     let s2 = String::from("hello");
//     let s3 = take_and_gives_back(s2);
//     println!("Value is s3 {s3}");
// }

// fn main() {
//     let s1: String = String::from("HELLO");
//     let (s2, value) = get_length(s1);
//     println!("Value is {s2} and length is {value}");
// }

// fn get_length(s1: String) -> (String, usize) {
//     let length_data = s1.len();
//     return (s1, length_data)
// }

// fn calculate_length(s: &String) -> usize {
//     return s.len();
// }

// fn main() {
//     let s1: String = String::from("Hello");
//     let len = calculate_length(&s1);
//     println!("Value this {}", len);
// }

// fn change_something(some_string: &mut String) {
//     some_string.push_str("Index");
// }

// fn main() {
//     let mut data: String = String::from("Omg!");
//     println!("Hello data {data}");
// }

// fn main() {
//     let mut s: String = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     let mut value_string = String::from("hello");
//     {
//         let value1 = &mut value_string;
//         println!("Value string {}", value1);
//     }
//     let value2 = &mut value_string;
//     println!("Value string {}", value2);
// }

// fn main() {
//     let mut s = String::from("Hello world");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("Value 1: {}\n Value 2: {}", r1, r2);

//     let value3 =  &mut s;
//     println!("Value 3: {}", value3);
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}", r2);
// }
// fn dangle() -> &'static str {
//     let s: &'static str = "Hello";
//     return s
// }

// fn main() {
//     println!("Value: {}", dangle());
// }

// fn dangle() -> &'static str {
//     let s: &'static str= ("OK Bro");
//     return s;
// }

// fn main() {
//     let reference_to_nothing = dangle();
//     println!("Value is {}", reference_to_nothing);
// }

// fn get_data() -> String {
//     let value_string = String::from("Ok, pro!!!");
//     return value_string;
// }

// fn main() {
//     let data = get_data();
//     println!("Value: {}", data);
// }

// fn get_data(s: &String) -> &[u8] {
//     let bytes = s.as_bytes();
//     return bytes;
    
// }

// fn main() {
//     let data: String = String::from("hello");
//     let value: &[u8] = get_data(&data);
//     println!("Data is {:?}", value);
// }

// fn main() {
//     let bytes: Vec<u8> = vec![b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd'];

//     for(index, &value) in bytes.iter().enumerate() {
//         println!("Value {value}, index {index}")
//     }
// }

// fn main() {
//     let mut string_value = String::from("Helloworld");
//     let data = get_data(&string_value);
//     println!("Value is {}", data);
// }

// fn get_data(string_value: &String) -> usize {
//     for (index, value) in string_value.chars().enumerate() {
//         if(value == ' ') {
//             return index
//         }
//     }
//     return string_value.len()
// }

// fn get_data(string_value: &String) -> usize {
//     let bytes = string_value.as_bytes();
//     for (index, & value) in bytes.iter().enumerate() {
//         if(value == b' ') {
//             return index
//         }
//     }
//     return bytes.len();
// }

// fn main () {
//     let mut string_value = String::from("OKello world!");
//     let data = first_word(&string_value);
//     println!("Value is {}", data);
//     string_value.clear();
//     // string_value.push_str("Hello");
//     println!("Value is after clear {}", string_value);
// }

// fn main() {
//     let mut string_value = String::from("Okello world!");
//     let data = first_word(&string_value);
//     println!("Value is {}", data);
//     string_value.clear();
// }


// fn first_word(string_value: &String) -> &str {
//     for(index, value) in string_value.chars().enumerate() {
//         if value == ' ' {
//             return &string_value[0..index]
//         }
//     }
//     return &string_value;
// }

// fn main() {
//     let value_string = String::from("hello world");

//     let hello = &value_string[0..5];
//     let world = &value_string[6..11];
//     let all = &value_string[..];
//     let mul = &value_string[0..2];
//     println!("Value: {}", hello);
//     println!("Value: {}", world);
//     println!("Value: {}", all);
//     println!("Value: {}", mul);
// }

// fn main() {
//     let value_string = String::from("hello world");
//     let lenth_value = value_string.len();
//     let value1 = &value_string[6..];
//     let value2 = &value_string[6..lenth_value];
//     println!("Value: {}", value1);
//     println!("Value: {}", value2);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     return &s[..]
// }

// struct Person {
//     name: String,
//     age: u8,
// }

// fn create_person(_name: String, _age: u8) -> Person {
//     let person: Person = Person {
//         name: _name,
//         age: _age,
//     };
//     return person;
// }

// fn main() {
//     let name: String = String::from("Le Vo");
//     let age: u8 = 10;
//     let person: Person = create_person(name, age);
//     println!("Person name is {}, age is {}", person.name, person.age);
// }

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
// }

// fn main() {
//     let rectangle_value = Rectangle{
//         width: 10,
//         height: 20
//     };
//     let current_data = rectangle_value.area();
//     println!("Value is {}", current_data)
// }

// trait Shape {
//     fn area(&self) -> u32;
//     fn perimeter(&self) -> u32;
// }

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// struct Circle {
//     radius: f64
// }

// struct Square{
//     side: f64
// }

// impl Shape for Rectangle {
//     fn area(&self) -> u32 {
//         let area = self.width * self.height;
//         return area;
//     }
//     fn perimeter(&self) -> u32 {
//         let perimeter: u32 = (self.width + self.height) * 2;
//         return perimeter;
//     }
// }

// impl Shape for Circle {
//     fn area(&self) -> u32 {
//         let value = 3.14159 * self.radius * self.radius;
//         let area = value as u32;
//         return area;
//     }
//     fn perimeter(&self) -> u32 {
//         let value: f64 = 3.14159 * self.radius * 2.0;
//         let perimeter: u32 = value as u32;
//         return perimeter;
//     }
// }

// impl Shape for Square {
//     fn area(&self) -> u32 {
//         let value = self.side * self.side;
//         let area = value as u32;
//         return area;
//     }
//     fn perimeter(&self) -> u32 {
//         let value: f64 = self.side * 4.0;
//         let perimeter: u32 = value as u32;
//         return perimeter;
//     }
// }

// fn main() {
//     let rectangle: Rectangle = Rectangle {
//         width: 10,
//         height: 20
//     };
//     let circle: Circle = Circle{
//         radius: 12.3
//     };
//     let square: Square = Square{
//         side: 100.0
//     };

//     let area_rectangle: u32 = rectangle.area();
//     let perimeter_rectangle: u32 = rectangle.perimeter();
//     println!("area_rectangle {}", area_rectangle);
//     println!("perimeter_rectangle {}", perimeter_rectangle);


//     let area_circle : u32= circle.area();
//     let perimeter_circle : u32= circle.perimeter();
//     println!("area_circle {}", area_circle);
//     println!("perimeter_circle {}", perimeter_circle);

//     let area_square : u32= square.area();
//     let perimeter_square : u32= square.perimeter();
//     println!("area_square {}", area_square);
//     println!("perimeter_square {}", perimeter_square);

//     if area_square < area_rectangle {
//         println!("Rectangle has a larger area than Square.");
//         return 
//     }
//     if area_square > area_rectangle {
//         println!("Square has a larger area than Rectangle.");
//         return 
//     }
//     println!("Rectangle and Square have the same area.");
// }

// struct Point {
//     x :f64,
//     y: f64
// }

// fn main() {
//     let origin: Point = Point{x: 12.1, y: 10.2};
//     let point: Point = Point{x: 12.12, y: 10.21};
//     println!("Origin : {},{}", origin.x, origin.y);
//     println!("Point : {},{}", point.x, point.y);
// }

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let width = 10;
//     let height = 20;
//     let rect = Rectangle { width, height };
//     println!("Value width is {}, Value height is {}", rect.width, rect.height);
// }

trait Bool {
    fn get_info(&self) -> String;
}
struct Math {
    title: String,
    author: String,
    pages: u32
}

impl Bool for Math {
    fn get_info(&self) -> String {
        let info: String = format!("Title {}, author {}, pages {}", self.title, self.author, self.pages);
        return info
    }
}

fn main() {
    let math_book = Math {
        title: "Math book".to_string(),
        author: "Vo Le".to_string(),
        pages: 200
    };
    let info: String = math_book.get_info();
    print!("Book: {}", info);
}