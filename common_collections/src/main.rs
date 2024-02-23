// fn main() {
//     let mut numbers: Vec<i32> = Vec::new();
//     numbers.push(21);
//     numbers.push(2121);
//     numbers.push(-1);
//     numbers.push(120);
//     numbers.insert(1, ----10);

//     for (index, number) in numbers.iter().enumerate() {
//         println!("index is {}, value is {}", index, number);
//     }

// }

// fn call_vector(numbers: Vec<i32>) -> Vec<i32> {
//     let mut empty_vector: Vec<i32> = Vec::new();
//     for (index, &value) in numbers.iter().enumerate() {
//         if value % 2 == 0 {
//             empty_vector.push(value);
//         }
//     }
//     return empty_vector;
// }

// fn max_vector(numbers: Vec<i32>) -> i32 {
//     let mut max_value = numbers.max();

// }

// fn main() {
//     let mut numbers = vec![1, 2, 3, 4, 5];
//     numbers.push(1);
//     println!("{:?}", numbers);
//     // let mut numbers: Vec<i32> = Vec::new();
//     // numbers.push(21);
//     // numbers.push(2121);
//     // numbers.push(-1);
//     // numbers.push(120);
//     // numbers.insert(1, ----10);

//     // let vector = call_vector(numbers);

//     // println!("Value amount is {:?}", vector);

// }

// fn get_info(numbers: Vec<i32>) {
//     let value = numbers.iter().collect::<Vec<_>>();
//     println!("Values are: {:?}", value);
// }

// fn get_max_value(numbers: Vec<i32>) -> Result<i32, String> {
//     if let Some(max_value) = numbers.iter().max() {
//         Ok(*max_value)
//     } else {
//         Err(String::from("Array index out of range"))
//     }
// }

// fn get_min_value(numbers: Vec<i32>) -> Result<i32, String> {
//     if let Some(min_value) = numbers.iter().min() {
//         Ok(*min_value)
//     } else {
//         Err(String::from("Array index out of range"))
//     }
// }

// fn remove_same_value(numbers: Vec<i32>) -> Vec<i32> {
//     let mut temp_vector: Vec<i32> = Vec::new();
//     for &value in &numbers {
//         if !temp_vector.contains(&value) {
//             temp_vector.push(value);
//         }
//     }
//     temp_vector.sort_by(|a, b| b.cmp(a));

//     return temp_vector;
// }

// fn reverve_vector(numbers: Vec<i32>) -> Vec<i32> {
//     let mut reversed: Vec<i32> = Vec::new();
//     reversed = reversed.into_iter().rev().collect();
//     return reversed;
// }

// fn continuity_check(numbers: Vec<i32>) -> bool {
//     if (numbers.is_empty()) {
//         return false;
//     }

//     for i in 1..numbers.len() {
//         if numbers[i] <= numbers[i - 1] {
//             return false;
//         }
//     }
//     return true;
// }

// fn find_most_frequent_element(numbers: Vec<i32>) -> i32 {
//     let length = numbers.len();
//     let mut mapping:HashMap<u8, u8> = HashMap::new();

//     for &num in numbers.iter() {

//     }
// }
// fn main() {
// let max_value = get_max_value(new_vector);
// match max_value {
//     Ok(max_value) => println!("Maximum value is: {}", max_value),
//     Err(error) => println!("Error is {}", error)
// }

// let new_vector: Vec<i32> = vec![1, 2, 1, 4, 3, 4, 5, 6, 9, 7, 8, 11, 9, 10];
// let empty_vector: Vec<i32> = Vec::new();

// let new_vector = remove_same_value(new_vector);
// get_info(new_vector);

// let mut new_vector: Vec<i32> = vec![1, 2, 1, 4, 3, 4, 5, 6, 9, 7, 8, 11, 9, 10];
// new_vector.reverse();
// println!("Values are: {}", new_vector)

// let mut new_vector: Vec<i32> = vec![1, 2, 1, 4, 3, 4, 5, -5, -6, 9, 7, 8, 11, 9, 10];
// let empty_vector: Vec<i32> = vec![];
// let value = get_min_value(empty_vector);
// match value {
//     Ok(value) => println!("Min value: {}", value),
//     Err(err) => println!("Error: {:?}", err)
// }

// let mut new_vector1: Vec<i32> = vec![1, 2, 1, 4, 3, 4, 5, -5, -6, 9, 7, 8, 11, 9, 10];
// let mut new_vector1: Vec<i32> = vec![-10, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11];

// let status = continuity_check(new_vector1);
// println!("Status: {:?}", status);

// let mut mapping: HashMap<u8, u8> = HashMap::new();
// mapping.insert(111, 123);
// println!("Mapping: {:?}", mapping.entry(111).or_insert(3));
// }

// use std::collections::HashMap;
// fn find_most_frequent_element(numbers: &Vec<u8>) -> (u8, u8) {
//     if numbers.is_empty() {
//         return (0, 0);
//     }

//     let mut frequency_map: HashMap<u8, u8> = HashMap::new();

//     for &num in numbers.iter() {
//         let count = frequency_map.entry(num).or_insert(0);
//         *count += 1;
//     }

//     let mut count_times = 0;
//     let mut frequent_number = numbers[0];

//     for &num in numbers.iter() {
//         if frequency_map[&num] > count_times {
//             count_times = frequency_map[&num];
//             frequent_number = num;
//         }
//     }

//     return (frequent_number, count_times);
// }

// fn main() {
//     let numbers = vec![1, 2, 3, 1, 3, 3, 3, 2, 1, 3, 4];
//     let value = find_most_frequent_element(&numbers);
//     println!("Time count {:?} and number {:?}", value.0, value.1);
// }

// fn main() {
//     let mut vector: Vec<i32> = Vec::new();
//     vector.push(1);
//     vector.push(2);
//     vector.push(3);
//     println!("Value is {:?}", vector);

//     let new_vector: Vec<u8> = vec![1, 3, 5];
//     print!("Value is {:?}", new_vector);
// }

// fn main() {
//     let vector = vec![1,2,3,4];

//     let value_option: Option<&i32> = vector.get(3);

//     match value_option {
//         Some(value) => println!("Values is {}", value),
//         None => println!("Not invalis value")
//     }
// }

// use std::fmt::format;

// fn main() {
//     let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
//     // let dose_not_exist = &vector[100];
//     let dose_nodoes_not_exist = vector.get(1);
//     match dose_nodoes_not_exist {
//         Some(value) => println!("Values is {}", value),
//         None => {
//             let error = format!("Error: Index {} is out of bounds", 100);
//             println!("Error: {}", error);
//         }
//     }
// }
