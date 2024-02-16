# RUST_BLOCKCHAIN_TUTORIAL
The Web3 Rust Course - NEAR Smart Contracts Web Development

## Commit type 
```
git commit -m "[Volh][Blockchain][Rust][Main functioin]"
```

### MAIN FUNCTIONS
```
fn main() {
    let value: i8 = 127;
    println!("Value", {value});
}
```

### PARSE
```
fn main() {
    let guess: u32 = "42".parse().expect("Guess is not a number");
    println!("Guess is {}", guess);
}
```

### The cargo new command
```
cargo new PROJECT_NAME
```
### to_owned
```
fn main() {
    let mut string_value: &str = "Hello, world!";
    let binding = (string_value.to_owned() + "world!");
    println!("binding: {}", binding);
}
```
### function in function 
```
fn print_stg(s: &str) {
    println!("print: {}", s);
}

fn main() {
    let my_str: &str = "Hello, Rust";
    let new_sth = my_str.to_owned() + "....No";
    println!("new_sth: {}", new_sth)
}
```

