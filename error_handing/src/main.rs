// fn main() {
//     println!("Hello, world!");
//     panic!("Crash and burn!")
// }
// fn main() {
//     let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
//     let value = v[9];
//     println!("Value is {:?}", value);
// }

// use std::fs::File;
// fn main() {
//     let greeting_file_result = File::open("./hello.txt");
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(err) => panic!("Have problem when opening file: {}", err)
//     };
//     println!("Value is {:?}", greeting_file)
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("./hello.txt");
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Have problem when creating file: {:?}", e)
//             }
//             other_error => {
//                 panic!("Have problem when creating file is {:?}", other_error)
//             }
//         }
//     };
//     println!("Value is: {:?}", greeting_file)
// }
// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello1.txt").unwrap();
//     println!("Value is {:?}", greeting_file);
// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt")
//         .expect("Open file hello.txt error, please check your folder");
//     println!("Value is {:?}", greeting_file);
// }

// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn main() {
//     match read_username_from_file() {
//         Ok(username) => {
//             println!("SUccess: {}", username);
//         }
//         Err(error) => {
//             eprintln!("Failed {:?}", error)
//         }
//     }
// }
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file {:?}", error)
//             })
//         } else {
//             panic!("Problem epening the file {:?}", error)
//         }
//     });

//     println!("Value is {:?}", greeting_file)

// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap()
//     println!("Vale")
// }

// use std::error::Error;
// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// fn main() -> Result<(), Box<dyn Error>> {
//     match read_username_from_file() {
//         Ok(content) => {
//             println!("Content of the file: {}", content);
//             Ok(())
//         }
//         Err(e) => Err(Box::new(e)),
//     }
// }

// #### To panic! or Not to panic!

#[derive(Debug)]
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value from 1 to 100, But current value is {}", value);
        }
        return Guess{value};
    }

    pub fn value(&self) -> i32 {
        return self.value;
    }
}

fn main() {
    let guess: Guess = Guess::new(12312);
    let current_value = guess.value();
    println!("Value is: {:?}", guess);
    println!("current_value is {:}", current_value);
}