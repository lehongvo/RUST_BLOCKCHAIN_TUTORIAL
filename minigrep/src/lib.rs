// use std::env::var;
// use std::error::Error;
// use std::fs;

// pub struct Config {
//     pub query: String,
//     pub file_path: String,
//     pub case_sensitive: bool,
// }

// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("Not enough arguments. Usage: minigrep <query> <file_path>");
//         }
//         let case_sensitive = var("CASE_INSENSITIVE").is_err();
//         let config = Config {
//             query: args[1].clone(),
//             file_path: args[2].clone(),
//             case_sensitive: case_sensitive,
//         };
//         return Ok(config);
//     }

//     pub fn read_file(&self) -> Result<String, std::io::Error> {
//         fs::read_to_string(&self.file_path)
//     }

//     pub fn search<'a>(&self, content: &'a str) -> Vec<&'a str> {
//         let value = content
//             .lines()
//             .filter(|line| line.contains(&self.query))
//             .collect();
//         return value;
//     }
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut result = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             result.push(line);
//         }
//     }
//     return result;
// }

// pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let query = query.to_lowercase();

//     let value = contents
//         .lines()
//         .filter(|line| line.to_lowercase().contains(&query))
//         .collect();
//     println!("Value is {:?}", value);
//     return value;
// }

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = std::fs::read_to_string(&config.file_path)?;
//     let results = if config.case_sensitive {
//         search(&config.query, &contents)
//     } else {
//         search_case_insensitive(&config.query, &contents)
//     };

//     for line in results.clone() {
//         println!("{}", line);
//     }

//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(
//             vec!["safe, fast, productive."],
//             search(query, contents)
//         );
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";
//         assert_eq!(
//             vec!["safe, fast, productive.", "Duct tape."],
//             search_case_insensitive(query, contents),
//         );
//     }
// }

use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep <query> <file_path>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
