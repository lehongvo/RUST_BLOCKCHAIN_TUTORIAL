pub mod froint_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist")
        }
    }
}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod customer {
    use crate::froint_of_house::hosting::add_to_waitlist;

    use crate::back_of_house::{Appetizer, Breakfast};

    pub fn eat_at_restaurant() {
        add_to_waitlist();

        let mut meal = Breakfast::summer("Rice");

        meal.toast = String::from("Wheat");
        println!("meal :{:?}", meal);

        let order2 = Appetizer::Salad;

        println!("Order 2 is {:?}", order2)
    }
}

use crate::customer::eat_at_restaurant;
use std::collections::HashMap;

fn get_balance(balances: &HashMap<String, u128>) {
    for (key, value) in balances {
        println!("Key : {key}, Value is {value}");
    }
}

// fn main() {
// //     let mut my_hash: HashMap<String, i32> = HashMap::new();
// //     my_hash.insert(String::from("0x123"), 1000);
// //     my_hash.insert(String::from("0x124"), 112312);
// //     my_hash.insert(String::from("0x125"), 345345);
// //     my_hash.insert(String::from("0x126"), 98273984);

// //     // if let Some(value) = my_hash.get("0x128") {
// //     //     println!("Value is {:?}", value);
// //     // } else {
// //     //     println!("Invalid data");
// //     // }

// //     // for (key, value) in &my_hash {
// //     //     println!("Key is {}, value is {}", key, value);
// //     // }
//        let mut balances: HashMap<String, u128> = HashMap::new();

//        balances.insert(String::from("user1Address"), 10000000000000000);
//        balances.insert(String::from("user2Address"), 1000000000000000011);

//        // get balance
//        get_balance(&balances);

//        println!("Data is {:?}", balances.remove("user2Address"));

//        get_balance(&balances);

// }

// use std::fmt;
// use std::io;

// fn print_number(number: i128) -> Result<i128, fmt::Error> {
//     if number > 0 {
//         Ok(number)
//     } else {
//         Err(fmt::Error)
//     }
// }

// fn write_to_file(text: &str, filename: &str) -> io::Result<()> {
//     use std::fs::File;
//     use std::io::Write;

//     let mut file = File::create(filename)?;

//     file.write_all(text.as_bytes())?;

//     Ok(())
// }

// fn main() {
//     match print_number(-10) {
//         Ok((value)) => println!("Valis {value} is ok"),
//         Err(_) => println!("Invalid number")
//     }

//     match write_to_file("Hello, Rust!", "output.txt") {
//         Ok(()) => println!("Ghi vào tệp tin thành công"),
//         Err(err) => println!("Có lỗi khi ghi vào tệp tin: {}", err),
//     }
// }

// use std::fmt;
// use std::io;

// fn print_number(number: i128) -> Result<i128, fmt::Error> {
//     if (number < 0) {
//         Ok(number)
//     } else {
//         Err(fmt::Error)
//     }
// }

// fn write_to_file(text: String, filename: String) -> io::Result<()> {
//     use std::fs::File;
//     use std::io::Write;

//     let mut file = File::create(filename)?;
//     file.write_all(text.as_bytes())?;

//     Ok(())
// }

// fn main() {
//     match print_number(-10) {
//         Ok(value) => println!("Valus is {value}"),
//         Err(_) => println!("Invalid number"),
//     }
//     match print_number(10) {
//         Ok(value) => println!("Valus is {value}"),
//         Err(_) => println!("Invalid number"),
//     }

//     match write_to_file(String::from("iam a monster1212"), String::from("data.json")) {
//         Ok(()) => println!("Print successfully"),
//         Err(_) => println!("Error"),
//     };
// }


use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    println!("Executing function 1");
    Ok(())
}

fn function2() -> IoResult<()> {
    println!("Executing function 2");
    Ok(())
}

fn main() {
    match function1() {
        Ok(()) => println!("Is ok 1"),
        Err(_) => println!("Error")
    }
    match function2() {
        Ok(()) => println!("Is ok 1"),
        Err(_) => println!("Error")
    }
}    