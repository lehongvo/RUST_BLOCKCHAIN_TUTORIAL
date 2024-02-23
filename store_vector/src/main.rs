// fn main() {
//     let vector = vec![1, 2, 3, 4];

//     let dose_not_exist = &vector.get(100);
//     match dose_not_exist {
//         Some(value) => println!("Value is {}", value),
//         None => println!("Error: Out of range from 0 to {}", vector.len()),
//     }
// }

// fn main () {
//     let mut vector = vec![1, 2, 3, 4];
//     let first_value = &vector.get(0).cloned();
//     vector.push(10);
//     println!("First element is {:?}", first_value);
// }

// fn main() {
//     let vector = vec![100, 32, 57];
//     for (index, value) in vector.iter().enumerate() {
//         println!("Value {index} is  {value}");
//     }
// }

// fn main() {
//     let mut vector = vec![100, 120, 123];
//     for i in &mut vector {
//         *i += 11;
//     }
//     println!("Value is: {:?}", vector);
// }

// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// fn main() {
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Float(4.5),
//         SpreadsheetCell::Text(
//             String::from("Vo Le Hong")
//         )
//     ];
//     println!("Value is {:?}", row)
// }

// fn main() {
//     {
//         let vector = vec![100, 12, 234];
//         for i in vector {
//             println!("Value is {:?}", i);
//         }
//     }
//     print!("vector: {:?}", vector)

// }

fn main() {
    // let my_str: &str = "Hello, world!";
    // let my_string: String = String::from("Hello, world");
    // println!("Value is {}", my_str);
    // println!("Value is {}", my_string);

    // let mut empty_string = String::new();

    // let data = "Initial contents";
    // let data_to_string = data.to_string();

    // let mut my_string = String::from("foo");
    // my_string.push_str("Hi");
    // println!("my_string {}", my_string)

    // let s1 = String::from("Hello,!");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("Values is s3 {}", s3);

    let hello = "Здравствуйте";
    for value in hello.chars() {
        println!("Value is {}", value);
    }
 }
