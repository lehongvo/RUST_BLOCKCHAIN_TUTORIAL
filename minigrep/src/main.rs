// use minigrep::{Config};
// use std::env;

// fn main() {
//     let agrs: Vec<String> = env::args().collect();
//     let config = Config::new(&agrs);

//     let contents: Result<String, std::io::Error> = config.read_file();

//     match contents {
//         Ok(contents) => {
//             let result = config.search(&contents);
//             println!("Content is \n{}", contents);
//             println!("Result is {:?}", result);
//         }
//         Err(_) => {
//             println!("Error now");
//             std::process::exit(1);
//         },
//     }
// }

// use std::env;
// use std::fs;
// use dotenv::dotenv;


// pub fn search<'a>(query: &'a str, contents: &'a str, case_insensitive: &bool) -> Vec<&'a str> {
//     let mut result = Vec::new();

//     println!("case_insensitive: {:?}", case_insensitive);

//     for line in contents.lines() {
//         if *case_insensitive && line.to_lowercase().contains(&query.to_lowercase()) {
//             result.push(line);
//         };
//     }

//     return result;
// }

// fn main() {
//     if let Err(error) = dotenv() {
//         eprintln!("Error loading .env file {:?}", error);
//         return;
//     }

//     let mut case_insensitive = false;
//     match env::var("MY_VARIABLE") {
//         Ok(value) => case_insensitive = value.parse().unwrap(),
//         Err(err) => println!("Invalid value: {:?}", err)
//     }

//     let args: Vec<String> = env::args().collect();
//     let query = &args[1];
//     let filename = &args[2];

//     let contents = fs::read_to_string(filename).expect("Can not reading this file");
//     let value = search(&query, &contents, &case_insensitive);
//     println!("value is {:?}", value);
// }

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}