// // fn main() {
// //     let total_fn = |x, y| x + y;
// //     let result = total_fn(10, 1);
// //     println!("Value is {:?}", result);
// // }

// // fn main() {
// //     let vector = vec![1, 3, 4, 6, 6];
// //     let max_value = vector.iter().max();

// //     match max_value {
// //         Some(value) => println!("Max value is {:?}", value),
// //         None => println!("Error now")
// //     }

// //     let sum_value: i32 = vector.iter().sum();
// //     println!("Sum value is {:?}", sum_value);
// // }


// // fn main() {
// //     let total_fn = |x: i32, y: i32| -> i32{
// //         let value = x + y;
// //         return value;
// //     };

// //     let mut_fn = |a: u128| -> u128 {
// //         let mut_value = a * a;
// //         return mut_value;
// //     };

// //     println!("Value is {:?}", mut_fn(1123123123123211201))
// // }

// // fn find_min_len(vector: Vec<&str>) {
// //     for value in vector {
// //         println!("String {} have length {}", value, value.len());
// //     }
// // }

// // fn main() {
// //     let string_vector = vec!["hello", "world", "rust", "iterator"];
// //     string_vector.iter().for_each(|string| {
// //         println!("Value: {}, have length {}", string, string.len());
// //     })
// // }

// // fn main() {
// //     let list_number = vec![1, 2, 3, 4, 5, 6, 7, 8];
// //     let empty_vectors: Vec<i32> = list_number
// //         .into_iter().filter(|&x| x % 2 == 0).collect();
// //     println!("Value: {:?}", empty_vectors)
// // }
// // #[derive(Debug, PartialEq, Copy, Clone)]
// // enum ShirtColor {
// //     Red,
// //     Blue
// // }
// // #[derive(Debug, PartialEq, Clone)]
// // struct Inventory {
// //     shirt: Vec<ShirtColor>
// // }

// // impl Inventory {
// //     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
// //         let value = user_preference.unwrap_or_else(|| self.most_stocked());
// //         return value;
// //     }

// //     fn most_stocked(&self) -> ShirtColor {
// //         let mut num_red = 0;
// //         let mut num_bule = 0;

// //         for color in &self.shirt {
// //             match color {
// //                 ShirtColor::Red => num_red += 1,
// //                 ShirtColor::Blue => num_bule += 1,
// //             }
// //         }
// //         if(num_red > num_bule) {
// //             return ShirtColor::Red;
// //         } else{
// //             return ShirtColor::Blue;
// //         }
// //     }
// // }

// // fn main() {
// //     let store = Inventory {
// //         shirt: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
// //     };
// //     let user_pref1 = None;
// //     let giveaway1 = store.giveaway(user_pref1);
// //     println!("Value is {:?}", giveaway1)
// // }

// // use std::thread;
// // use std::time::Duration;

// // fn main() {
// //     let expensive_close = |num: u32| -> u32 {
// //         println!("Calculating slowly...");
// //         thread::sleep(Duration::from_secs(10));
// //         return num;
// //     };

// //     let result = expensive_close(10);
// //     println!("Value is {:?}", result);
// // }

// // fn main() {
// //     let total_now = |x: u32| -> u32 {
// //         let value = x + 1;
// //         return x;
// //     };
// // }

// // use std::thread;

// // fn main() {
// //     let mut list = vec![1, 2, 3, 4, 5];
// //     println!("Before defining closure: {:?}", list);

// //     thread::spawn(move || println!("From thread: {:?}", list))
// //         .join()
// //         .unwrap();
// // }

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // fn main() {
// //     let mut list = [
// //         Rectangle { width: 10, height: 1 },
// //         Rectangle { width: 3, height: 5 },
// //         Rectangle { width: 7, height: 12 },
// //     ];

// //     let mut num_sort_operations = 0;

// //     list.sort_by_key(|r| {
// //         num_sort_operations+ 1;
// //         r.width
// //     });
// //     println!("{:#?}", list);
// // }

// // fn main() {
// //     let data = vec![1, 2, 3];
// //     let read_only = || {
// //         for value in data {
// //             println!("Value is {:?}", value)
// //         }
// //     };
// //     read_only();
// // }

// // fn main() {
// //     let mut value = 0;
// //     let mut increment_value = || {
// //         value = value + 1;
// //         println!("Value is {:?}", value);
// //     };
// //     increment_value();
// //     increment_value();
// //     increment_value();
// //     increment_value();
// // }

// // use std::thread;

// // fn main() {
// //     let data = vec![1, 2, 3];

// // }

// // fn main() {
// //     let vector = vec![1, 2, 4];
// //     let vector_inter = vector.iter().f

// // }

// // fn main() {
// //     let vector = vec![1, 2, 3];
// //     for value in vector.iter() {
// //         println!("Values is {}", value);
// //     }
// // }

// // #[cfg(test)]
// // pub mod tests {
// //     #[test]
// //     pub fn iterator_demonstration() {
// //         let v1 = vec![1, 2, 3];
// //         let mut v1_inter = v1.iter();
// //         assert_eq!(v1_inter.next(), Some(&1), "Invalid value");
// //         assert_eq!(v1_inter.next(), Some(&2), "Invalid value");
// //         assert_eq!(v1_inter.next(), Some(&3), "Invalid value");
// //     }
// // }

// // fn iterator_sum() {
// //     let v1 = vec![1, 2, 3];


// //     let total: i32 = v1.iter().sum();

// //     assert_eq!(total, 6, "Invalid valid");
// // }

// // fn main() {
// //     let vector: Vec<i32> = vec![1, 2, 3];
// //     println!("New vector: {:?}", vector);
// //     let new_vector: Vec<i32> = vector.iter().map(|&x| x + 3).collect();
// //     println!("Old value: {:?}", new_vector);
// // }
// // #[derive(Debug, PartialEq)]
// // struct Shoe {
// //     size: u32,
// //     style: String
// // }

// // fn shose_in_size(shoes_list: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
// //     let new_shoes: Vec<Shoe> = shoes_list.into_iter().filter(|value| value.size == shoe_size).collect();
// //     return new_shoes;
// // }

// // #[cfg(test)]
// // pub mod tests {
// //     use super::*;
// //     #[test]
// //     fn filter_by_size() {
// //         let shoes_list = vec![
// //             Shoe {
// //                 size: 10,
// //                 style: String::from("sneaker"),
// //             },
// //             Shoe {
// //                 size: 13,
// //                 style: String::from("sandal"),
// //             },
// //             Shoe {
// //                 size: 10,
// //                 style: String::from("boot"),
// //             },
// //         ];
// //         let in_my_size = shose_in_size(shoes_list, 13);

// //         assert_eq!(
// //             in_my_size,
// //             vec![
// //                 Shoe{
// //                     size: 13,
// //                     style:String::from("sandal")
// //                 }
// //             ],
// //             "Invalid data"
// //         );
// //     }
// // }

// use std::env;

// pub struct Config {
//     pub query: String,
//     pub file_path: String,
//     pub ignore_case: bool
// }

// impl Config {
//     pub fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         let ignore_case = env::var("IGNORE_CASE").is_ok();

//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//         })
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     match Config::build(&args) {
//         Ok(config) => {
//             // Do something with the config
//             println!("Query: {}", config.query);
//             println!("File Path: {}", config.file_path);
//             println!("Ignore Case: {}", config.ignore_case);
//         }
//         Err(err) => {
//             eprintln!("Error: {}", err);
//             std::process::exit(1);
//         }
//     }
// }

