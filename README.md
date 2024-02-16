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

### full_str is mean cut char form string
```
fn main() {
    let full_str: &str = "Hello, Rust!";
    let part_str: &str = &full_str[0..6];
    println!("part_str {}", part_str)
}
```

### push_str is mean push new value to String data type
```
fn main() {
    let mut my_string: String = String::from("Hello, Rust!");
    my_string.push_str("Hello, Rust!");
    println!("my_string, {}", my_string);
}
```
### remove + insert only one char in  a string type 
```
fn main() {
    let mut my_string: String = String::from("Hello, Rust!");
    my_string.remove(1);
    my_string.insert(1, 'E');
    println!("my_string, {}", my_string);
}
```

### usize using for length array

```
fn main() {
    let mut my_string: String = String::from("Hello, Rust!");
    let length_value: usize = my_string.len();
    println!("Value length, {}", length_value);
}
```

### is_empty using for check is string or stg is empty or not
```
fn main() {
    let is_empty_str: &str = "";
    let is_empty_string: String = String::from("Hello, Rust!");
    let is_right_str: bool = is_empty_str.is_empty();
    let is_right_strong: bool = is_empty_string.is_empty();
    println!("is_right_str: {}", is_right_str);
    println!("is_right_str: {}", is_right_strong);
}
```

### contains in array, include to_uppercase on RUST
```
fn main() {
    let is_contain_str: &str =  "Hello, world";
    let is_contain_strong: String = String::from("Ok pro");
    if 
      is_contain_str.to_uppercase().contains(&("OK").to_uppercase()) && 
      is_contain_strong.to_uppercase().contains(&("HELLO").to_uppercase())
    {
        println!("Yes {} is correctly contained", is_contain_str);
    } else {
        println!("No is not correctly contained");
    }
}
```

### float 32 bit and 64 bit
```
fn main () {
    let current_float: String = String::from("9.9999999999999911");
    let value_float: f64 = current_float.parse().expect("Not a number float");
    println!("value_float: {}", value_float)
}
```

### String::new ampty string
```
fn main() {
    let mut string_new: String = String::new();
    string_new.push('O');
    string_new.push_str("LE HONG VO");
    println!("string_new: {}", string_new);
}
```