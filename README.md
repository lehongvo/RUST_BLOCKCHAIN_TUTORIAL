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

### Reading input
```
use std::io;
fn main() {
    println!{"Guess the number!"};
    println!("Please input your guess.");
    let mut you_number: String = String::new();
    io::stdin()
        .read_line(&mut you_number)
        .expect("Error reading");
    println!("Please input your guess.");
    println!("You guessed: {}", you_number)
}
```

### Random number
```
use rand::Rng;
fn main() {
    let mut totalAmount: i128 = 0;
    for n in 1..=100 {
        let mut rng = rand::thread_rng();
        let random_number:i128 = rng.gen_range(-1..100000000000999900000099999990);
        println!("Random number: {}", random_number);
        totalAmount += random_number * 99999;
    }
    println!("Total amount: {}", totalAmount);
}
```
#### Get random number
```
use rand::Rng;
use std::io;
fn main() {
    println!("PLease guess my number!!!\n");
    println!("PLease input your number from(0 to u8)");

    // Input number
    let mut system_input: String = String::new();
    io::stdin()
        .read_line(&mut system_input)
        .expect("Error reading");

    // Read number
    let system_value: u8 = match system_input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Đầu vào không hợp lệ. Vui lòng nhập kiểu dữ u8");
            return;
        }
    };

    // Ramdon number
    let mut count_time: u128 = 0;
    for n in 0..=1000000 {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0..127);
        println!("Random number is: {}", random_number);
        if (random_number == system_value) {
            count_time += 1;
        } 
    }
    println!("You win {} times", count_time);
}
```

### Total number cal functions
```
use std::io;
fn read_number(prompt: &str) -> u128 {
    println!("Please enter {}", prompt);
    let mut value: String = String::new();
    io::stdin().read_line(&mut value).expect("Error reading");
    let value_input: u128 = match value.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalis number");
            std::process::exit(1)
        }
    };
    return value_input;
}

fn main() {
    let value_one: u128 = read_number("Value 1");
    let value_two: u128 = read_number("Value 2");
    let total_value: u128 = value_one + value_two;
    println!("total_value is: {}", total_value);
}

```