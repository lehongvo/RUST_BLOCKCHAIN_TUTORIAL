// Filename: src/lib.rs

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 6, "=====Compare failed====One");
//     }
    
//     #[test]
//     fn new() {
//         assert!(2 == 4, "=====Compare failed====Two");
//     }

//     // #[test]
//     // fn another() {
//     //     panic!("This test will fail");
//     // }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::Rectangle;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger: Rectangle = Rectangle{
//             width: 8,
//             height: 7
//         };
        
//         let smaller: Rectangle =  Rectangle{
//             width: 5,
//             height: 1
//         };

//         assert!(larger.can_hold(&smaller), "==Can not hole rectangles larger than==")
//     }
// }


// #[cfg(test)]
// mod test {
//     #[test]
//     fn check_value() {
//         let x = 10;
//         let y = 20;
//         assert_eq!(x, y, "==Ohh, it not correct==");
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     let value = a + 1;
//     return value;
// }

// #[cfg(test)]
// pub mod check_fn {
//     use super::add_two;
    
//     #[test]
//     pub fn is_adds_two() {
//         assert_eq!(4, add_two(0), "==Ohh error come==")
//     }
// }

// pub fn greeting(name: &str) -> String {
//     let string = format!("Hello, I am a {}", name);
//     return string;
// }

// #[cfg(test)]
// pub mod test1 {
//     use super::*;
    
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Ca1rol");
//         let is_ok = result.contains("Carol");
//         assert!(
//             is_ok,
//             "====Greeting did not contain name, value was `{}`", result
//         )
//     }
// }


// pub struct Guess {
//     value: i32
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         let value: Guess = Guess{value};
//         return value;
//     }
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;

//     #[test]
//     #[should_panic()]
//     fn greater_than_100() {
//         Guess::new(300);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// #[cfg(test)]
// mod tests_01 {
//     #[test]
//     fn some_operation_returns_err() -> Result<(), String> {
//         let result  = some_operation();
//         println!("Value is {}", result.is_err());
//         assert!(false, "--------------------------------ERROR");
//         Ok(())
//     }
    
//     #[test]
//     fn some_operation() -> Result<(), String> {
//         // Logic of some_operation that might return Err
//         Ok(())
//     }
// }

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     let a = 10;
//     return a;
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(1000);
//         assert_eq!(value, 5, "=======Value not correct");
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(99);
//         assert_eq!(value, 10, "=======Value not correct")
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     println!("value for input: {}", a);
//     let a = a + 2;
//     return a;
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;

//     #[test]
//     fn add_two_and_two() {
//         assert_eq!(4, add_two(2), "Invalid value from: add_two_and_two")
//     }

//     #[test]
//     fn add_three_and_two() {
//         assert_eq!(5, add_two(3), "Invalid value from: add_two_and_two");
//     }

//     #[test]
//     #[ignore]
//     fn add_value() {
//         assert_eq!(5, add_two(3), "Invalid value from: add_two_and_two");
//     }

//     #[test]
//     pub fn one_hundred() {
//         assert_eq!(102, add_two(100), "Invalid value from: one_hundred");
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4, "Value is no correct");
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     let value = internal_adder(a, 2);
//     return value;
// }

// fn internal_adder(a: i32, b: i32) -> i32 {
//     println!("You call me");
//     let total_value = a + b;
//     return total_value;
// }

// #[cfg(test)]
// pub mod check {
//     use super::*;

//     #[test]
//     fn internal() {
//         assert_eq!(4, internal_adder(2, 2));
//     }
// }

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}