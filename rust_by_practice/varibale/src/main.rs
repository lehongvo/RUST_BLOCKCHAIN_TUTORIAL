// =================================================================

// Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32 = 5; // Uninitialized but used, ERROR !
//     let _y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fill the blanks in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x += 2;

//     assert_eq!(x, 3);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix the error below with least amount of modification
// fn main() {
//     let x: i32 = 10;
//     let mut y = 10;
//     {
//         y = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is {}", x, y);
// }
// =================================================================

// =================================================================

// Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", define_x());
// }

// fn define_x() -> String {
//     let x = "Hello";
//     return x.to_string();
// }
// =================================================================

// =================================================================

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }
//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x);
// }
// =================================================================

// =================================================================

// Remove a line in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     let mut x = x;
//     x += 3;

//     let y = 4;
//     let y = "I can also be bound to text!";

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     let _x = 1;
// }

// Warning: unused variable: `x`
// =================================================================

// =================================================================

// Fix the error below with least amount of modification
// fn main() {
//     let (mut x, mut y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     assert_eq!([x,y], [3,2]);

//     // println!("Success!");
// }
// =================================================================

// =================================================================

// Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     y = x as u32;

//     let z = 10; // Type of z ?

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fill the blank
// fn main() {
//     let v: u16 = 38_u128 as u16;
//     println!("Value is {}", v);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Modify `assert_eq!` to make it work
// Modify `assert_eq!` to make it work
// fn main() {
//     let x:u32 = 5;
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// =================================================================

// =================================================================
// Fill the blanks to make it work
// Fix errors and panics to make it work
// fn main() {
//     let v1 = 251_u8 + 8;
//     let v2 = i8::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }
// =================================================================

// =================================================================
// Fix errors and panics to make it work
// fn main() {
//     let v1: u16 = (251_u16 + 8).into();
//     let v2: i16 = i16::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }
// =================================================================

// =================================================================
// Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     println!("Value is {}", v);
//     assert!(v == 1597, "Assertion failed: Expected 1579, but got {}", v);
//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fill the blank to make it work
// fn main() {
//     let x = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }
// =================================================================

// =================================================================
// fn main() {
//     let value = 0.1 as f32 + 0.2 as f32;
//     println!("value is {}", value);
//     assert!(value == 0.3 as f32);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }
// =================================================================

// =================================================================
// Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fill the blanks and fix the errors
// fn main() {
//     // Integer addition
//     assert!(1u32 + 2u32 == 3u32);

//     // Integer subtraction
//     assert!(1i32 - 2i32 == -1);
//     assert!(1i8 - 2i8 == -1i8);

//     assert!(3 * 50 == 150);

//     assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

//     assert!(24 % 5 == 4);
//     // Short-circuiting boolean logic
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }
// =================================================================

// =================================================================

// use std::mem::size_of_val;

// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),4);

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),4);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Make println! work
// fn main() {
//     let _f: bool = false;

//     let t = true;
//     if t {
//         println!("Success!");
//     }
// }
// =================================================================

// =================================================================
// // Make it work, don't modify `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());

//     println!("Success!");
// }

// fn implicitly_ret_unit() -> (i32, i32){
//     println!("I will return a ()");
//     return (2, 3);
// }

// // Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }
// =================================================================

// =================================================================
// Modify `4` in assert to make it work
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // This expression will be assigned to `y`
//         x_cube + x_squared + x
//     };

//     println!("Y value is {}", y);
//     println!("Y value is {}", x);

//     let z = {
//         // The semicolon suppresses this expression and `()` is assigned to `z`
//         2 * x;
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };

//     assert_eq!(v, 3);

//     println!("Success!");
//  }
// =================================================================

// =================================================================
// fn main() {
//     let v = {let x = 3; x};

//     assert!(v == 3);

//     println!("Success!");
//  }
// =================================================================

// =================================================================
// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }
// =================================================================

// use std::fmt::format;

// =================================================================
// fn main() {
//     let value = print("VoLeHong");
//     println!("Value is {:?}", value);
// }

// // Replace i32 with another type
// fn print(name: &str) -> String {
//     let value = format!("Hello, world! my name is {name}");
//     return value;
// }
// =================================================================

// =================================================================
// Solve it in two ways
// DON'T let `println!` work
// fn main() {
//     never_return();

//     println!("Failed!");
// }

// fn never_return() -> ! {
//     panic!("=====Nerver error!=====");
//     // Implement this function, don't modify the fn signatures
// }
// =================================================================

// =================================================================

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!("Printing error");
// }
// =================================================================

// =================================================================

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!();
// }
// =================================================================

// =================================================================
// fn main() {
//     // FILL in the blank
//     let b = true;

//     let _v = match b {
//         true => 1,
//         // Diverging functions can also be used in match expression to replace a value of any value
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic");
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }
// =================================================================

// =================================================================
// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("Hello world");
//     let y = &x;
//     println!("{}, {}",x, y);
// }
// =================================================================

// =================================================================
// Don't modify code in main!
// fn main() {
//     let s1 = String::from("Hello world");
//     let s2 = take_ownership(&s1);

//     println!("{:?}", &s2);
// }

// // Only modify the code below!
// fn take_ownership(s: &String) {
//     println!("{}", s);
// }
// =================================================================

// =================================================================
// fn main() {
//     let s = give_ownership();
//     println!("{:?}", s);
// }

// // // Only modify the code below!
// fn give_ownership() -> Vec<u8> {
//     let s = String::from("Hello world");
//     // Convert String to Vec
//     let _s = s.into_bytes();
//     _s
// }
// =================================================================

// =================================================================
// // Fix the error without removing any code
// fn main() {
//     let s = String::from("Hello World");

//     print_str(&s);

//     println!("{}", s);
// }

// fn print_str(s: &String)  {
//     println!("{}",s)
// }
// =================================================================

// =================================================================
// // Don't use clone ,use copy instead
// fn main() {
//     let x: (i32, i32, (), &str) = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }
// =================================================================

// =================================================================
// make the necessary variable mutable
// fn main() {
//     let s = String::from("Hello ");

//     let mut s1 = s;

//     s1.push_str("World!");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let x = Box::new(5);

//     let mut y = Box::new(1);// update this line, don't change other lines!

//     *y = 4;

//     assert_eq!(*x, 5);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // `name` is moved out of person, but `age` is referenced
//     let Person { name, ref age} = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     // Error! borrow of partially moved value: `person` partial move occurs
//     //println!("The person struct is {:?}", person);

//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age);
// }
// =================================================================

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     let _s = &t.0;

//     // Modify this line only, don't use `_s`
//     println!("{:?}", t);
//  }

// =================================================================

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//      // Fill the blanks
//      let (s1, s2) = (&t.0, &t.1);

//      println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//  }
// =================================================================

// =================================================================
// fn main() {
//     let x = 5;
//     let p = &x;

//     println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
//  }
// =================================================================

// =================================================================

// fn main() {
//     let x = 5;
//     let y = &x;

//     // Modify this line only
//     assert_eq!(5, *y);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fix error
// fn main() {
//     let mut _s = String::from("hello, ");

//     borrow_object(&_s);

//     println!("Success!");
// }

// fn borrow_object(s: &String) {}
// =================================================================

// =================================================================
// fn main() {
//     let mut s = String::from("hello, ");

//     // Fill the blank to make it work
//     let p = &mut s;

//     p.push_str("world");

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     let c = '中';

//     let r1 = &c;
//     // Fill the blank，dont change other code
//     let ref r2 = c;

//     assert_eq!(*r1, *r2);

//     // Check the equality of the two address strings
//     assert_eq!(get_addr(r1),get_addr(r2));

//     println!("Success!");
// }

// // Get memory address string
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }
// =================================================================

// =================================================================

// Remove something to make it work
// Don't remove a whole line !
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     // Fix error by modifying this line
//     let  s = String::from("hello, ");

//     borrow_object(&s);

//     println!("Success!");
// }

// fn borrow_object(s: &String) {}
// =================================================================

// =================================================================
// This code has no errors!
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s);

//     s.push_str("world");

//     println!("Success!");
// }

// fn borrow_object(s: &String) {}
// =================================================================

// =================================================================
// // Comment one line to make it work
// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");

//     // println!("{}",r1);
// }
// =================================================================

// =================================================================
// Comment one line to make it work
// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
//     // You can't use r1 and r2 at the same time
//     println!("{}, {}", r1, r2);
// }
// =================================================================

// =================================================================
// Fix error without adding new line
// fn main() {
//     let s: &str = "hello, world";

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }
// =================================================================

// =================================================================
// Fill the blank
// fn main() {
//     let mut s = String::from("Ok pro, ");
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix all errors without adding newline
// fn main() {
//     let mut s: String = String::from("hello");
//     s.push(',');
//     s.push_str("world");
//     s += "!";

//     println!("{}", s);
// }
// =================================================================

// =================================================================

// Fill the blank
// fn main() {
//     let mut s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

// Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2;
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s3);
// }
// =================================================================

// =================================================================

// Fix error with at least two solutions
// fn main() {
//     let s = "hello, world";
//     greetings(s.to_string())
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }
// =================================================================

// =================================================================

// Use two approaches to fix the error and without adding a new line
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = s.as_str();

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     // You can use escapes to write bytes by their hexadecimal values
//     // Fill the blank below to show "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );

//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }
// =================================================================

// =================================================================

/* Fill in the blank and fix the errors */
// fn main() {
//     let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//     // Modify above line to make it work
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

//     // If you need quotes in a raw string, add a pair of #s
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // If you need "# in your string, just use more #s in the delimiter.
//     // You can use up to 65535 #s.
//     let delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);

//     // Fill the blank
//     let long_delimiter = __;
//     assert_eq!(long_delimiter, "Hello, \"##\"");

//     println!("Success!");
// }
// use std::str;

// fn main() {
//     // Note that this is not actually a `&str`
//     let bytestring: &[u8; 21] = b"this is a byte string";
//     println!("A byte string: {:?}", bytestring);

//     // Byte strings can have byte escapes...
//     let escaped = b"\x52\x75\x73\x74 as bytes";
//     // ...But no unicode escapes
//     // let escaped = b"\u{211D} Is not allowed";
//     println!("Some escaped bytes: {:?}", escaped);

//     // Raw byte strings work just like raw strings
//     let raw_bytestring = br"\u{211D} is not escaped here";
//     println!("{:?}", raw_bytestring);

//     // Converting a byte array to `str` can fail
//     if let Ok(my_str) = str::from_utf8(raw_bytestring) {
//         println!("And the same as text: '{}'", my_str);
//     }
//     // Byte strings don't have to be UTF-8
//     let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" In SHIFT-JIS

//     // But then they can't always be converted to `str`
//     match str::from_utf8(shift_jis) {
//         Ok(my_str) => println!("Conversion successful: '{}'", my_str),
//         Err(e) => println!("Conversion failed: {:?}", e),
//     };
// }
// =================================================================

// =================================================================
// Byte strings don't have to be UTF-8

// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
//     assert_eq!(h, "h");

//     let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
//     assert_eq!(h1, "中");

//     println!("Success! {}", h1);
// }

// =================================================================

// =================================================================
//  The function prints each character in the string "你好，世界" in Rust.
// fn main() {
//     for c in ("你好，世界").chars() {
//         println!("{}", c)
//     }
// }
// =================================================================

// =================================================================
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";

//     let rocket = s.split(s, 4, 5);
//     // Will equal "🚀"
// }

// =================================================================

// =================================================================
// fn main() {
//     let arr = [1; 2];
//     println!("Value is {:?}", arr);
// }
// =================================================================

// =================================================================
// fn main() {
//     // Fill the blank with proper array type
//     let arr: [u8; 5] = [1, 2, 3, 4, 5];

//     // Modify the code below to make it work
//     assert!(arr.len() == 5);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     // Fill the blank with proper array type
//     let arr: __ = [1, 2, 3, 4, 5];

//     // Modify the code below to make it work
//     assert!(arr.len() == 4);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     // Fill the blank
//     let list: [i32; 2] = [1,2];

//     assert!(list[0] == 1);
//     assert!(list.len() == 2);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     // Fix the error
//     let _arr = [1, 2, -3];

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     let arr = ['a', 'b', 'c'];
//     let ele = arr[0];
//     println!("Value is {}", ele);
//     assert!(ele == 'a');
//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix the error
// Fix the error
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];

//     // `Get` returns an Option<T>, it's safe to use
//     let name0 = names.get(0).unwrap();

//     // But indexing is not safe
//     let _name1 = &names[1];

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix the errors, DON'T add new lines!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1 = &arr[0..2];

//     let s2: &str = "hello, world";

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];

//     let slice = &arr[..2];

//     // Modify '8' to make it work
//     // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
//     assert!(std::mem::size_of_val(&slice) == 16);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // Fill the blanks to make the code work
//     let slice = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let s = String::from("hello");

//     let slice1 = &s[0..2];
//     // Fill the blank to make the code work, DON'T USE 0..2 again
//     let slice2 = &s[..2];

//     assert_eq!(slice1, slice2);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let s = ("你好，世界").to_string();
//     // Modify this line to make the code work
//     let slice = &s[0..3];

//     println!("Value is {}", slice);

//     assert!(slice == "你");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix errors
// fn main() {
//     let mut s = String::from("hello world");

//     // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
//     // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
//     let letter = first_letter(&s);
//     println!("the first letter is: {}", letter);
//     s.clear(); // error!

// }
// fn first_letter(s: &str) -> &str {
//     &s[..1]
// }
// =================================================================

// =================================================================
// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // Tuples can be tuple's members
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     // Fill the blanks to make the code work
//     let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

//     println!("Success! with value :{:?}", t);
// }
// =================================================================

// =================================================================
// Make it work
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix the error
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13);
//     println!("too long tuple: {:?}", too_long_tuple);
// }
// =================================================================

// =================================================================
// fn main() {
//     let tup = (1, 6.4, "hello");

//     // Fill the blank to make the code work
//     let (x, z, y) = tup;

//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let (x, y, z);

//     // Fill the blank
//     (y, z, x) = (1, 2, 3);

//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     // Fill the blank, need a few computations here.
//     let (x, y) = sum_multiply((2, 3));

//     assert_eq!(x, 5);
//     assert_eq!(y, 6);

//     println!("Success!");
// }

// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn main() {
//     let user1 = User {
//         active: false,
//         username: "Le Hong Vo".to_string(),
//         email: "lehongvi19x@gmail.com".to_string(),
//         sign_in_count: 812
//     };

//     println!("Value is {:?}", user1);

//     let user1 = User {
//         active: true,
//         ..user1
//     };
//     println!("Value is {:?}", user1);
// }
// =================================================================

// =================================================================

// Fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: String::from("Hangout")
//     };

//     println!("Success!");
// }
// =================================================================

// =================================================================
// struct Unit;
// trait SomeTrait {
//     // ...Some behaviors defined here.
// }

// // We don't care about what fields  are  in the Unit, but we care about its behaviors.
// // So we use a struct with no fields and implement some behaviors for it
// impl SomeTrait for Unit {  }
// fn main() {
//     let u = Unit;
//     do_something_with_unit(u);

//     println!("Success!");
// }

// // Fill the blank to make the code work
// fn do_something_with_unit(u: Unit) {   }
// =================================================================

// =================================================================

// Fill the blank to make the code work
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let u1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let u2 = set_email(u1);
//     println!("User value: {:?}", u2);

//     println!("Success!");
// }

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }
// =================================================================

// =================================================================

// Fill the blanks to make the code work
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
//         height: 50,
//     };

//     dbg!(&rect1); // Print debug info to stderr

//     println!("Rectangle is {:?}", rect1); // Print debug info to stdout
// }
// =================================================================

// =================================================================
// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // `name` is moved out of person, but `age` is referenced
//     let Person { name, ref age } = &person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     // Error! borrow of partially moved value: `person` partial move occurs
//     //println!("The person struct is {:?}", person);

//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", &person.age);
// }
// =================================================================

// =================================================================

// // Fix errors to make it work
// #[derive(Debug)]
// struct File {
//     name: String,
//     data: String,
// }
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string()
//     };

//     let _name = &f.name;

//     // ONLY modify this line
//     println!("{}, {}, {:?}",&f.name, &f.data, &f);
// }
// =================================================================

// =================================================================
// Fix the errors
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One = 1,
//     Two,
// }

// // C-like enum
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 3,
// }

// fn main() {
//     assert_eq!(Number::One as i32, Number1::One as i32);
//     assert_eq!(Number1::One as u8, Number2::One as u8);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fill in the blank
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg1 = Message::Move{x: 12, y: 12};
//     let msg2 = Message::Write(String::from("LE HONG VO"));
//     println!("Success!");
// }
// =================================================================

// =================================================================
// // Fill in the blank and fix the error
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::Move{x: 1, y: 1};

//     if let Message::Move{x: a, y: b} = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("NEVER LET THIS RUN！");
//     }

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fill in the blank and fix the errors
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// }

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }
// =================================================================

// =================================================================
// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     if let Some(n) = six {
//         println!("{}", n);

//         println!("Success!");
//     }

//     panic!("NEVER LET THIS RUN！");
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
// =================================================================

// =================================================================

// use crate::List::*;

// enum List {
//     Cons(u32, Box<List>),
//     Nil,
// }

// // Methods can be attached to an enum
// impl List {
//     fn new() -> List {
//         Nil
//     }

//     fn prepend(self, elem: u32) -> List {
//         Cons(elem, Box::new(self))
//     }

//     fn len(&self) -> u32 {
//         match *self {
//             Cons(_, ref tail) => 1 + tail.len(),
//             Nil => 0
//         }
//     }

//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {

//                 format!("{}, {}", head, tail.stringify())
//             }
//             Nil => {
//                 format!("Nil")
//             },
//         }
//     }
// }

// fn main() {
//     let mut list = List::new();
//     println!("{}", list.stringify());

//     list = list.prepend(1);
//     println!("{}", list.stringify());
//     list = list.prepend(2);
// }
// =================================================================

// =================================================================

// Fill in the blanks
// fn main() {
//     let n = 5;

//     if n < 0 {
//         println!("{} is negative", n);
//     } if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// }
// =================================================================

// =================================================================

// Fill in the blanks
// fn main() {
//     let n = 5;

//     let big_n = if n < 10 && n > -10 {
//         println!(", and is a small number, increase ten-fold");

//         10 * n
//     } else {
//         println!(", and is a big number, halve the number");

//         n / 2
//     };

//     println!("{} -> {}", n, big_n);
// }
// =================================================================

// =================================================================

// fn main() {
//     for n in 1..100 { // modify this line to make the code work
//         if n == 100 {
//             panic!("NEVER LET THIS RUN")
//         } else {
//             println!("Ok from {}", n)
//         }
//     }

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fix the errors without adding or removing lines
// fn main() {
//     let names = [String::from("liming"),String::from("hanmeimei")];
//     for name in &names {
//         println!("{}", name);
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // The elements in numbers are Copy，so there is no move here
//     for n in numbers {
//         println!("Value is {}", n)
//     }

//     println!("{:?}", numbers);
// }
// =================================================================

// =================================================================
// fn main() {
//     let a = [4, 3, 2, 1];

//     for (i,v) in a.iter().enumerate() {
//         println!("The {}th element is {}",i+1,v);
//     }
// }
// =================================================================

// =================================================================
// Fill in the blanks to make the last println! work !
// fn main() {
//     let mut n = 1;

//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//         n += 1;
//     }

//     println!("n reached {}, so loop is over",n);
// }
// =================================================================

// =================================================================

// Fill in the blank
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n == 66 {
//           println!("OMG");
//           break;
//        }
//        n += 1;
//     }
//     assert_eq!(n, 66);
//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fill in the blanks

// // Fill in the blanks
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//        if n != 66 {
//            n+=1;
//            continue;
//        }
//        break;
//     }

//     assert_eq!(n, 66);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// // Fill in the blanks
// fn main() {
//     let mut count = 0u32;

//     println!("Let's count until infinity!");

//     // Infinite loop
//     loop {
//         count += 1;

//         if count == 3 {
//             println!("three");

//             // Skip the rest of this iteration
//             continue;
//         }

//         println!("{}", count);

//         if count == 5 {
//             println!("OK, that's enough");

//             break;
//         }
//     }

//     assert_eq!(count, 5);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fill in the blank
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     assert_eq!(result, 20);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Fill in the blank
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 break 'inner1;
//             }
//             count += 2;
//         }

//         count += 5;

//         'inner2: loop {
//             if count >= 30 {
//                 break 'outer;
//             }

//             continue 'outer;
//         }
//     }

//     println!("Value is {count}");

//     assert!(count == __);

//     println!("Success!");
// }

// =================================================================

// =================================================================
// fn main() {
//     let value = Some(19);
//     if let Some(value) = value {
//         println!("Value is {value}");
//     }
// }
// =================================================================

// =================================================================
// Fill the blanks
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => { // Matching South or North here
//             println!("South or North");
//         },
//         _ => println!("Other direction"),
//     };
// }
// =================================================================

// =================================================================
// fn main() {
//     let boolean = true;

//     // Fill the blank with a match expression:
//     //
//     // boolean = true => binary = 1
//     // boolean = false =>  binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 2
//     };

//     assert_eq!(binary, 1);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }

//     println!("Success!");
// }

// fn show_message(msg: Message) {
//     match msg {
//         Message::Move{x :a, y: b} => { // match  Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         __ => println!("no data in these variants")
//     }
// }
// =================================================================

// =================================================================
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0' | '9'));
//     }

//     println!("Success!");
// }
// =================================================================

// =================================================================
// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if let MyEnum::Foo = &e {
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     let o = Some(7);

//     // Remove the whole `match` block, using `if let` instead
//     match o {
//         Some(i) => {
//             println!("This is a really long string and `{:?}`", i);

//             println!("Success!");
//         }
//         _ => {}
//     };
//     if let Some(value) = &o {
//         println!("This is a really long string and `{:?}`", value);
//         println!("Success!");
//     } else {
//         println!("Failure!");
//     }
// }
// =================================================================

// =================================================================
// Fill in the blank
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     match a {
//         Foo::Bar(i) => {
//             println!("foobar holds the value: {}", i);

//             println!("Success!");
//         },
//         _ => print!("Error")
//     }
// }
// =================================================================

// =================================================================
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a = Foo::Qux(10);

//     // Remove the codes below, using `match` instead
//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match foo::bar")
//     }
// }
// =================================================================

// =================================================================
// Fix the errors in-place
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // Create a new variable with the same name as previous `age`
//        assert_eq!(age, 30);
//     } // The new variable `age` goes out of scope here

//     match age {
//         // Match can also introduce a new shadowed variable
//         Some(age) =>  println!("age is a new variable, it's value is {}", &age),
//         _ => ()
//     }
//  }
// =================================================================

// =================================================================

// fn main() {
//     match_number(11)
// }
// fn match_number(n: i32) {
//     match n {
//         // Match a single value
//         1 => println!("One!"),
//         // Fill in the blank with `|`, DON'T use `..` or `..=`
//         2..=5 => println!("match 2 -> 5"),
//         // Match an inclusive range
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match -infinite -> 0 or 11 -> +infinite")
//         }
//     }
// }
// =================================================================

// use std::process::id;

// // =================================================================
// enum Message {
//     Hello { id: i32 },
// }
// fn main() {
//     let msg = Message::Hello { id: 5 };

//     match msg {
//         Message::Hello {id} => {
//             if (3..=7).contains(&id) {
//                 println!("Tìm thấy id trong khoảng [3, 7]: {}", id);
//             }
//         }
//         Message::Hello {id} => {
//             if (10..=12).contains(&id) {
//                 println!("Tìm thấy id trong khoảng khác [10, 12]: {}", id);
//             }
//         }
//         Message::Hello { id } => {
//             println!("Tìm thấy một id khác: {}", id);
//         }
//     }
// }
// =================================================================

// =================================================================

// Fill in the blank to make the code work, `split` MUST be used
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) => {
//             if x < split {
//                 assert!(x < split);
//             }
//         }
//         Some(x) => {
//             if x > split {
//                 assert!(x >= split)
//             }
//         },
//         None => (),
//     }
//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fill the blank to make the code work
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

//     match numbers {
//         (first,..,last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }

//     println!("Success!");
// }
// =================================================================

// =================================================================

// FIX the error with least changing
// DON'T remove any code line
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;

//     match r {
//        value => value.push_str(" world!")
//     }
// }
// =================================================================

// =================================================================
// struct Point {
//     x: f64,
//     y: f64,
// }

// impl Point {
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }

//     fn new(x: f64, y: f64) -> Point {
//         Point { x: x, y: y }
//     }
// }

// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }

// impl Rectangle {

//     fn area(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;

//         ((x1 - x2) * (y1 - y2)).abs()
//     }

//     fn perimeter(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;

//         2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
//     }

//     fn translate(&mut self, x: f64, y: f64) {
//         self.p1.x += x;
//         self.p2.x += x;

//         self.p1.y += y;
//         self.p2.y += y;
//     }
// }

// struct Pair(Box<i32>, Box<i32>);

// impl Pair {

//     fn destroy(self) {
//         let Pair(first, second) = self;
//         println!("Destroying Pair({}, {})", first, second);

//     }
// }

// fn main() {
//     let rectangle = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(3.0, 4.0),
//     };

//     // Methods are called using the dot operator.
//     // Note that the first argument `&self` is implicitly passed, i.e.
//     // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
//     println!("Rectangle perimeter: {}", rectangle.perimeter());
//     println!("Rectangle area: {}", rectangle.area());

//     let mut square = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(1.0, 1.0),
//     };

//     // Error! `rectangle` is immutable, but this method requires a mutable
//     // object.
//     //rectangle.translate(1.0, 0.0);
//     // TODO ^ Try uncommenting this line

//     // Okay! Mutable objects can call mutable methods
//     square.translate(1.0, 1.0);

//     let pair = Pair(Box::new(1), Box::new(2));

//     pair.destroy();

//     // Error! Previous `destroy` call "consumed" `pair`
//     //pair.destroy();
//     // TODO ^ Try uncommenting this line
// }
// =================================================================

// =================================================================
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> i32 {
//         let area = self.width * self.height;
//         return area as i32;
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     assert_eq!(rect1.area(), 1500);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Only fill in the blanks, DON'T remove any line!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light: TrafficLight = TrafficLight{
//         color: "red".to_owned(),
//     };
//     light.show_state();
//     println!("{:?}", light);
// }
// =================================================================

// =================================================================
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // Using `Self` to fill in the blank.
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }

//     // Fill in the blank, DON'T use any variants of `Self`.
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }
// fn main() {
//     println!("Success!");
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 1. Implement an associated function `new`,
//     // 2. It will return a TrafficLight contains color "red"
//     // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
//     pub fn new() -> TrafficLight {
//         let traffic_light = TrafficLight {
//             color: "red".to_string()
//         };
//         return traffic_light;
//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");

//     println!("Success!");
// }
// =================================================================

// =================================================================

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// // Using multiple `impl` blocks to rewrite the code below.
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     println!("Success!");
// }
// =================================================================

// =================================================================

// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// // Implement TrafficLightColor with a method.
// impl TrafficLightColor {
//     pub fn color(&self) -> String {
//         match &self{
//             TrafficLightColor::Red => "red".to_string(),
//             TrafficLightColor::Yellow => "yellow".to_string(),
//             TrafficLightColor::Green => "green".to_string(),
//         }
//     }
// }

// fn main() {
//     let c = TrafficLightColor::Yellow;

//     assert_eq!(c.color(), "yellow");

//     println!("{:?}",c);
// }
// =================================================================

// =================================================================
// Fill in the blanks to make it work
// struct A;
// struct S(A);
// struct SGen<T>(T);

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     reg_fn(S(A));
//     gen_spec_t(SGen(A));
//     gen_spec_i32(SGen(10));

//     generic::<char>(SGen('a'));

//     generic(SGen(10));

//     println!("Success!");
// }
// =================================================================

// =================================================================

// // Fill in the blanks to make it w/// The code defines various functions using concrete and generic
/// types in Rust, demonstrating both implicit and explicit type
/// parameter specification.
///
/// Arguments:
///
/// * `_s`: Here's a breakdown of the parameters used in the code
/// snippet:
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(-1)); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('a'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen(12));

//     println!("Success!");
// }
// =================================================================

// use std::ops::Add;

// // =================================================================
// fn sum<T>(value0: T, value1: T) -> T where T: Add<Output = T>{
//     let value = value0 + value1;
//     return value;
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }
// =================================================================

// =================================================================

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     println!("integer is {:?}", integer);

//     let float = Point { x: 1.0, y: 4.0 };
//     println!("float is {:?}", float);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Modify this struct to make the code work
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: "H!".to_string(), y : "hello".to_string()};

//     println!("Success!");
// }
// =================================================================

// =================================================================
// // Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }
// =================================================================

// =================================================================
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         let point = Point {
//             x: self.x,
//             y: other.y
//         };
//         return point;
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }
// =================================================================

// =================================================================
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup<K, L>(self, other: Point<K, L>) -> Point<T, L> {
//         let other_point = Point {
//             x: self.x,
//             y: other.y
//         };
//         return other_point;
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }
// =================================================================

// use std::{process::Output, ops::Add};

// // =================================================================
// // Implement the generic function below.
// fn sum<T>(value1: T, value2: T) -> T where T: Add<Output = T>{
//     let total_value = value1 + value2;
//     return total_value;
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Implement struct Point to make it work.
// struct Point<T, K> {
//     x: T,
//     y: K
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Modify this struct to make the code work
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: "hi".to_string(), y : "hello".to_string()};

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Modify this struct to make the code work

// Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }
// =================================================================

// =================================================================
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<K, J>(self, other: Point<K, J>) -> Point<T, J> {
//         Point{
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }
// =================================================================

// =================================================================
// Fix the errors to make the code work.

// Fix the errors to make the code work.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p: Point<f64> = Point{x: 5.0, y: 10.0};
//     println!("{}",p.distance_from_origin());
// }
// =================================================================

// =================================================================
// fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
//     let mut value = 0;
//     for element in &arr {
//         value += element;
//     }
//     return value;
// }
// fn main() {
//     let array1 = [1, 2, 3, 4, 5];
//     let value = sum_array(array1);
//     println!("Value is {:?}", value);
// }
// =================================================================

// =================================================================
// fn sort_array<const N: usize>(mut arr: [i32; N]) -> [i32; N] {
//     arr.sort();
//     return arr;
// }

// fn main() {
//     // Sử dụng hàm sắp xếp với mảng có kích thước cố định
//     let array = [5, 2, 8, 1, 7];
//     let sorted_array = sort_array(array);
//     println!("Sorted array: {:?}", sorted_array);
// }
// =================================================================

// =================================================================
// fn get_total<const N: usize>(values: [i32; N]) -> i32 {
//     let total_value = values.iter().sum();
//     return total_value;
// }

// fn main() {
//     let array = [5, 2, 8, 1, 7];
//     let sorted_array = get_total(array);
//     println!("Sorted array: {:?}", sorted_array);
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct ArrayPair<T, const N: usize> {
//     left: [T; N],
//     right:[T; N]
// }

// fn main() {
//     let value = ArrayPair {
//         left: [1, 2, 3],
//         right: [1, 2, 3],
//     };
//     println!("Value: {:?}", value);
// }
// =================================================================

// =================================================================
// fn foo<const N: usize>() {}
// fn bar<T, const M: usize>() {
//     foo::<M>;
//     foo::<2021>;
//     foo::<{20 * 100 + 20 * 10 + 1}>();
//     let _: [u8, m];

// }
// =================================================================

// =================================================================
// pub struct MinSlice<T, const N: usize> {
//     pub head: [T; N],
//     pub tail: [T]
// }

// fn main() {
//     let slice: &[u8] = b"Hello, world";
//     let reference = slice.get(6);
//     assert!(reference.is_some());
//     let slice: &[u8] = b"Hello, world";
//     let min_slice = MinSlice::<u8, 12>.
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct Array<T, const N: usize> {
//     data : [T; N]
// }

// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 4]
//         }
//     ];

//     println!("Success! is value {:?},", arrays);
// }
// =================================================================

// =================================================================
// Fill in the blanks to make it work.
// use std::fmt::Debug;

// fn print_array<T: Debug, const N: usize>(arr:[T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }
// =================================================================

// =================================================================
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }

// // Fix the errors in main.
// fn main() {
//     check_size([0u8; 767]);
//     check_size([0i32; 191]);
//     check_size(["hello你好"; __]); // Size of &str ?
//     check_size([(); __].map(|_| "hello你好".to_string()));  // Size of String?
//     check_size(['中'; __]); // Size of char ?

//     println!("Success!");
// }

// pub enum Assert<const CHECK: bool> {}

// pub trait IsTrue {}

// impl IsTrue for Assert<true> {}
// =================================================================

// =================================================================

// #[derive(Debug)]
// struct Sheep {
//     naked: bool,
//     name: String,
// }

// trait Animal {
//     // Associated function signature; `Self` refers to the implementor type.
//     fn new(name: String) -> Self;

//     // Method signatures; these will return a string.
//     fn name(&self) -> String;

//     fn noise(&self) -> String;

//     // Traits can provide default method definitions.
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }

//     fn un_naked(&mut self) {
//         self.naked = true;
//     }

//     fn shear(&mut self) {
//         if self.is_naked() {
//             // Implementor methods can use the implementor's trait methods.
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);

//             self.naked = true;
//         }
//     }
// }

// // Implement the `Animal` trait for `Sheep`.
// impl Animal for Sheep {
//     // `Self` is the implementor type: `Sheep`.
//     fn new(name: String) -> Sheep {
//         Sheep {
//             name: name,
//             naked: false,
//         }
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         if self.is_naked() {
//             "baaaaah?".to_string()
//         } else {
//             "baaaaah!".to_string()
//         }
//     }

//     // Default trait methods can be overridden.
//     fn talk(&self) {
//         // For example, we can add some quiet contemplation.
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

// fn main() {
//     // Type annotation is necessary in this case.
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());
//     // TODO ^ Try removing the type annotations.

//     dolly.talk();
//     dolly.shear();
//     dolly.talk();

//     dolly.un_naked();
//     dolly.shear();
//     dolly.noise();
// }
// =================================================================

// =================================================================

// trait Hello {
//     fn say_hi(&self) -> String;

//     fn say_something(&self) -> String;
// }

// #[derive(Debug)]
// struct Student {}

// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String {
//         let value = format!("I'm a good student");
//         return value;
//     }
// }

// #[derive(Debug)]
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//     fn say_something(&self) -> String {
//         let value = format!("I'm not a bad teacher");
//         return value;
//     }
// }

// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!");
// }
// =================================================================

// =================================================================

// #[derive(PartialEq, Debug, PartialOrd)]
// struct Centimeters(f64);

// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// #[derive(PartialEq, Debug, PartialOrd)]
// struct Seconds(i32);

// fn main() {
//     let _one_second = Seconds(1);

//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = (_one_second == _one_second);
//     let _this_is_false = (_one_second > _one_second);

//     let foot = Inches(12);

//     println!("One foot equals {:?}", foot);

//     let meter = Centimeters(100.0);

//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };

//     println!("One foot is {} than one meter.", cmp);
// }
// =================================================================

// =================================================================
// use std::ops::{self, Mul};

// fn multiply<T>(x: T, y: T) -> T where T: Mul<Output = T>{
//     let value = x * y;
//     return value;
// }

// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
//     println!("Success!");
// }

// =================================================================

// =================================================================

// Fix the errors, DON'T modify the code in `main`.
// use std::ops;

// #[derive(Debug, PartialEq)]
// struct Foo;

// #[derive(Debug, PartialEq)]
// struct Bar;

// #[derive(Debug, PartialEq)]
// struct FooBar;

// #[derive(Debug, PartialEq)]
// struct BarFoo;

// // The `std::ops::Add` trait is used to specify the functionality of `+`.
// // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// // The following block implements the operation: Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }

// fn main() {
//     // DON'T modify the code below.
//     // You need to derive some trait for FooBar to make it comparable.
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// pub struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// pub struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     post.summarize();
//     weibo.summarize();

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }
// =================================================================

// =================================================================
// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the errors here, you can make a fake random, or you can use trait object.
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }
// =================================================================

// use std::{process::Output, ops::Add};

// =================================================================
// fn main() {
//     assert_eq!(sum(1, 2), 3);
//     println!("All ok");
// }

// fn sum<T>(x: T, y: T) -> T  where T: Add<Output = T>{
//     x + y
// }
// =================================================================

// =================================================================
// FIX the errors.
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }

// #[derive(Debug, PartialEq, PartialOrd)]
// struct Unit(i32);

// fn main() {
//     let pair = Pair{
//         x: Unit(1),
//         y: Unit(3)
//     };

//     pair.cmp_display();
// }
// =================================================================

// =================================================================

// Fill in the blanks to make it work
// fn example1() {
//     // `T: Trait` is the commonly used way.
//     // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
//     struct Cacher<T: Fn(u32) -> u32> {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 }
//             }
//         }
//     }

//     let mut cacher = Cacher::new(|x| x + 1);
//     assert_eq!(cacher.value(10), __);
//     assert_eq!(cacher.value(15), __);
// }

// fn example2() {
//     // We can also use `where` to construct `T`
//     struct Cacher<T>
//     where
//         T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T> Cacher<T>
//     where
//         T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 }
//             }
//         }
//     }

//     let mut cacher = Cacher::new(|x| x + 1);
//     assert_eq!(cacher.value(20), __);
//     assert_eq!(cacher.value(25), __);
// }

// fn main() {
//     example1();
//     example2();

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn example_test1() {
//     struct Cached<T: Fn(u32) -> u32> {
//         calculate: T,
//         value: Option<u32>
//     }

//     impl <T: Fn(u32) -> u32> Cached<T> {
//         fn new(calculate: T) -> Cached<T> {
//             let new_cacher = Cached {
//                 calculate: calculate,
//                 value: None
//             };
//             return new_cacher;
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculate)(arg);
//                     self.value = Some(v);
//                     v
//                 }
//             }
//         }
//     }
//     let mut cached = Cached::new(|x| x + 1);
//     assert_eq!(cached.value(10), 11);
//     assert_eq!(cached.value(15), 15);

// }

// fn main() {
//     example_test1();
// }
// =================================================================

// =================================================================
// trait Printable {
//     fn print(&self);
// }

// struct Dog;

// impl Printable for Dog {
//     fn print(&self) {
//         println!("This a dog");
//     }
// }

// fn print_trait_object(printable: &dyn Printable) {
//     printable.print();
// }

// fn main() {
//     let dog = Dog;
//     print_trait_object(&dog);
// }

// =================================================================

// =================================================================
// trait Bird {
//     fn quack(&self) -> String;
// }

// #[derive(Debug)]
// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }

// #[derive(Debug)]
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String{
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String{
//         "swan swan".to_string()
//     }
// }

// fn hatch_a_bird(id: i32) -> Box<dyn Bird> {
//     match id % 2 {
//         0 => Box::new(Duck),
//         _ => {
//             println!("OK");
//             Box::new(Swan)
//         },
//     }
// }

// fn main() {
//     let duck = Duck {};
//     duck.swim();

//     let bird: Box<dyn Bird> = hatch_a_bird(2);
//     assert_eq!(bird.quack(), "duck duck");

//     let swan = hatch_a_bird(1);
//     assert_eq!(swan.quack(), "swan swan");

//     println!("Success!");
// }

// =================================================================

// =================================================================
// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     // FILL in the blank to make the code work.
//     let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

//     for bird in birds {
//         bird.quack();
//         // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
//         // So, the code below will cause an error.
//         // bird.fly();
//     }
// }
// =================================================================

// =================================================================

// FILL in the blanks.
// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn main() {
//     let x = 1.1f64;
//     let y = 8u8;

//     // Draw x.
//     draw_with_box(Box::new(x));

//     // Draw y.
//     draw_with_ref(&y);

//     println!("Success!");
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw_with_ref(x: &dyn Draw) {
//     x.draw();
// }
// =================================================================

// =================================================================
// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }

// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }

// // IMPLEMENT below with generics.
// fn static_dispatch<T : Foo>(value: T) {
//     println!("{}", value.method());
// }

// // Implement below with trait objects.
// fn dynamic_dispatch(value: &dyn Foo) {
//     println!("{}", value.method());
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Use at least two approaches to make it work.
// DON'T add/remove any code line.
// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42u32 }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }

// fn my_function<T: MyTrait>(x: T) {
//     println!("Print some thing");
// }

// fn main() {
//     my_function(21);
//     my_function("Je".to_string());

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Use at least two approaches to make it work.
// DON'T add/remove any code line.
// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }

// fn my_function<T: MyTrait>(x: T)  {
//     println!("Print some thing");
// }

// fn main() {
//     my_function(21);
//     my_function("Je".to_string());

//     println!("Success!");
// }
// =================================================================

// =================================================================

// Use at least two approaches to make it work.
// DON'T add/remove any code line.
// trait MyTrait {
//     fn f(&self) -> Box<dyn MyTrait>;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Box<dyn MyTrait> { Box::new(342) }
// }

// impl MyTrait for String {
//     fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
// }

// fn my_function(x: Box<dyn MyTrait>)  {
//     println!("Do some thing");
//     // x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("Success!");
// }
// =================================================================

// =================================================================
// struct Container(i32, i32);

// trait Contains<A, B> {
//     fn contains(&self, _: &A, _: &B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains<i32, i32> for Container {
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }
//     fn first(&self) -> i32 { self.0 }

//     fn last(&self) -> i32 { self.1 }
// }

// fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;

//     let container = Container(number_1, number_2);

//     println!("Does container contain {} and {}: {}",
//         &number_1, &number_2,
//         container.contains(&number_1, &number_2));
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());

//     println!("The difference is: {}", difference(&container));
// }

// =================================================================

// use std::ops::Sub;

// // =================================================================
// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T
// }
// impl <T: Sub<Output = T>> Sub for Point<T> {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Point {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }
// fn main(){
//     let point = Point{x: 0, y:10} - Point{x: 0, y:11};
//     println!("Value is {:?}", point);
// }
// =================================================================
// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T
// }

// impl <T: Mul<Output = T>> Mul for Point<T> {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         let point = Point {
//             x: self.x * rhs.x,
//             y: self.y * rhs.y
//         };
//         return point;
//     }
// }
// fn main() {
//     let point = Point{x: 3, y: 10} * Point{x: 4, y: 9};
//     println!("Value is: {:?}", point);
// }
// =================================================================

// =================================================================
// use std::ops::{Add, Mul};

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T: Add<Output = T>> Add for Point<T> {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         let new_point = Point {
//             x: self.x + rhs.x,
//             y:  self.y + rhs.y
//         };
//         return new_point;
//     }
// }

// fn main() {
//     let p1 = Point {x: 1, y: 2};
//     let p2 = Point {x: 3, y: 43};
//     let p2 = p1 + p2;
//     println!("Value is {:?}", p2);
// }
// =================================================================

// #[derive(Debug)]
// struct Container(i32, i32);

// trait Contains<A, B> {
//     fn contains(&self, number_1: &A, number_2: &B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains<i32, i32> for Container {
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         let value = (&self.0 == number_1) && (&self.1 == number_2);
//         return value;
//     }
//     // Grab the first number.
//     fn first(&self) -> i32 {
//         self.0
//     }

//     // Grab the last number.
//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;

//     let container = Container(number_1, number_2);

//     println!(
//         "Does container contain {} and {}: {}",
//         &number_1,
//         &number_2,
//         container.contains(&number_1, &number_2)
//     );
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());

//     println!("The difference is: {}", difference(&container));
// }

// =================================================================
// use std::ops::Sub;

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl <T: Sub<Output = T>> Sub for Point<T> {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("Success!");
// }
// =================================================================

// =================================================================
// trait UsernameWidget {
//     // Get the selected username out of this widget
//     fn get(&self) -> String;
// }

// trait AgeWidget {
//     // Get the selected age out of this widget
//     fn get(&self) -> u8;
// }

// // A form with both a UsernameWidget and an AgeWidget.
// #[derive(Debug)]
// struct Form {
//     username: String,
//     age: u8,
// }

// impl UsernameWidget for Form {
//     fn get(&self) -> String {
//         self.username.clone()
//     }
// }

// impl AgeWidget for Form {
//     fn get(&self) -> u8 {
//         self.age
//     }
// }

// fn main() {
//     let form = Form{
//         username: "rustacean".to_owned(),
//         age: 28,
//     };

//     // If you uncomment this line, you'll get an error saying
//     // "multiple `get` found". Because, after all, there are multiple methods
//     // named `get`.
//     // println!("{}", form.get());

//     let username = UsernameWidget::get(&form);
//     println!("Value is {}", username);
//     assert_eq!("rustacean".to_owned(), username);
//     let age = AgeWidget::get(&form); // You can also use `<Form as AgeWidget>::get`
//     assert_eq!(28, age);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// trait Pilot {
//     fn fly(&self) -> String;
// }
// trait Wizard {
//     fn fly(&self) -> String;
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) -> String {
//         String::from("This is your captain speaking.")
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) -> String {
//         String::from("Up!")
//     }
// }

// impl Human {
//     fn fly(&self) -> String {
//         String::from("*waving arms furiously*")
//     }
// }

// fn main() {
//     let person = Human;

//     assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
//     assert_eq!(Wizard::fly(&person), "Up!");

//     assert_eq!(person.fly(), "*waving arms furiously*");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// trait Person {
//     fn name(&self) -> String;
// }

// trait Student: Person {
//     fn university(&self) -> String;
// }

// trait Programmer {
//     fn fav_language(&self) -> String;
// }

// trait CompSciStudent: Programmer + Student {
//     fn git_username(&self) -> String;
// }

// fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
//     format!(
//         "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
//         student.name(),
//         student.university(),
//         student.fav_language(),
//         student.git_username()
//     )
// }

// struct CSStudent {
//     name: String,
//     university: String,
//     fav_language: String,
//     git_username: String
// }

// impl CompSciStudent for CSStudent {
//     fn git_username(&self) -> String {
//         self.git_username.clone()
//     }
// }

// // Làm tương tự cho các phương thức của trait Person, Student và Programmer
// impl Person for CSStudent {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
// }

// impl Student for CSStudent {
//     fn university(&self) -> String {
//         self.university.clone()
//     }
// }

// impl Programmer for CSStudent {
//     fn fav_language(&self) -> String {
//         self.fav_language.clone()
//     }
// }

// fn main() {
//     let student = CSStudent {
//         name: "Sunfei".to_string(),
//         university: "XXX".to_string(),
//         fav_language: "Rust".to_string(),
//         git_username: "sunface".to_string()
//     };

//     println!("{}", comp_sci_student_greeting(&student));
// }
// =================================================================

// =================================================================
// use std::fmt;

// // DEFINE a newtype `Pretty` here
// #[derive(Debug)]
// struct Pretty(String);

// fn main() {
//     let w = Pretty("hello".to_string());
//     println!("w = {:?}", w);
// }
// =================================================================

// =================================================================

// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
// fn main() {
//     let mut s = "hello, ".to_string();
//     s.push_str("world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");
//     move_ownership(s);

// }

// fn move_ownership(s: String) {
//     println!("ownership of \"{}\" is moved here!", s)
// }
// =================================================================

// =================================================================
// FILL in the blanks
// fn main() {
//     let mut s = String::from("hello, world");

//     let slice1: &str = &s ; // In two ways
//     assert_eq!(slice1, "hello, world");

//     let slice2 = &slice1[0..5];
//     assert_eq!(slice2, "hello");

//     let mut slice3 = s.to_owned();
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");

//     println!("Success!");
//  }

// =================================================================
// Question: how many heap allocations are happening here?
// Your answer:
// fn main() {
//     // Create a String type based on `&str`
//     // The type of string literals is `&str`
//    let s: String = String::from("hello, world!");

//    // Create a slice point to String `s`
//    let slice: &str = &s;

//    // Create a String type based on the recently created slice
//    let s: String = slice.to_string();

//    assert_eq!(s, "hello, world!");

//    println!("Success!");
// }
// =================================================================

// =================================================================

// FILL in the blank and FIX errors
// fn main() {
//     let s = String::from("hello, 世界");
//     let slice1: &str = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
//     assert_eq!(slice1, "h");

//     let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
//     assert_eq!(slice2, "世");

//     // Iterate through all chars in s
//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }

//     // println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//    let s = "The 🚀 goes to the 🌑!";

//    let rocket = &s[4..8];
//    // Will equal "🚀"
//    println!("Value is {}", rocket)
// }
// =================================================================

// =================================================================

// FILL in the blanks
// fn main() {
//     let mut s = String::new();
//     s.push_str("hello");

//     // Some bytes, in a vector
//     let v = vec![104, 101, 108, 108, 111];

//     // Turn a byte's vector into a String
//     let s1 = String::from_utf8(v).expect("Failled to convert to String");

//     assert_eq!(s, s1);

// }
// =================================================================

// =================================================================

// Modify the code below to print out:
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
// fn main() {
//     let mut s = String::new();
//     println!("{}", s.capacity());

//     for _ in 0..2 {
//         s.push_str("hello");
//         println!("{}", s.capacity());
//     }

//     println!("Success!");
// }
// =================================================================

// =================================================================

// // FILL in the blanks
// use std::mem::{self, ManuallyDrop};

// fn main() {
//     let story = String::from("Rust By Practice");

//     // Prevent automatically dropping of the String's data
//     let mut story = ManuallyDrop::new(story);
//     println!("Value is {:?}", story);

//     let ptr = story.as_mut_ptr();
//     let len = story.len();
//     let capacity = story.capacity();

//     assert_eq!(16, len);

//     // We can rebuild a String out of ptr, len, and capacity. This is all
//     // unsafe because we are responsible for making sure the components are
//     // valid:
//     let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

//     assert_eq!(*story, s);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];

//     let v = Vec::from(arr);
//     is_vec(&v);

//     let v = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) and vec![..] are same macros, so
//     let v = vec!(1, 2, 3);
//     is_vec(&v);

//     // In code below, v is Vec<[u8; 3]> , not Vec<u8>
//     // USE Vec::new and `for` to rewrite the below code
//     let v1 = Vec::from(arr);
//     is_vec(&v1);

//     assert_eq!(v, v1);

//     println!("Success!");
// }

// fn is_vec(v: &Vec<u8>) {}
// =================================================================

// =================================================================

// FILL in the blank
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);

//     let mut v2 = Vec::new();
//     v2.extend(&v1);

//     assert_eq!(v1, v2);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// FILL in the blanks

// FILL in the blanks
// fn main() {
//     // Array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();

//     assert_eq!(v1, v2);

//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     // Iterators can be collected into vectors
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!");
//  }
// =================================================================

// =================================================================
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v[i])
//     }

//     for i in 0..5 {
//        // IMPLEMENT the code here...
//     }

//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// FILL in the blanks
// fn main() {
//     // Array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr: [i32; 3] = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();

//     assert_eq!(v1, v2);

//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     // Iterators can be collected into vectors
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!");
//  }
// =================================================================

// =================================================================
// FIX the error and IMPLEMENT the code
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..3 {
//         println!("{:?}", &v[i])
//     }

//     for i in 0..5 {
//         if i < v.len(){
//             v[i] = v[i] + 1;
//         } else {
//             v.push(i + 2);
//         }
//     }

//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// FIX the errors

// FIX the errors
// fn main() {
//     let mut v = vec![1, 2, 3];

//     let slice1 = &v[..];

//     let slice2 = &v[0..3];

//     assert_eq!(slice1, slice2);

//     // Slices are read only
//     // Note: slice and &Vec are different
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[..];
//     // slice3.push(4);

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// FIX the errors
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     // The vector contains no items, even though it has capacity for more
//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     // These are all done without reallocating...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     // ...but this may make the vector reallocate
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 20);

//     // Fill in an appropriate value to make the `for` done without reallocating
//     let mut vec = Vec::with_capacity(10);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 10);

//     // println!("Success!");
// }
// =================================================================

/// The function creates a vector of boxed values of any type and attempts to downcast the first value
/// to an `i32`, printing the value if successful.
// use std::any::Any;

// // =================================================================
// fn main() {
//     let vector_box: Vec<Box<dyn Any>> = vec![Box::new(1), Box::new(1.3), Box::new(-1)];
//     if let Some(value) = vector_box[0].downcast_ref::<i32>() {
//         println!("Value is: {}", value);
//     } else {
//         println!("Invalid valus");
//     }
// }
// =================================================================

// =================================================================
// #[derive(Debug, PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // FILL in the blank
//     let v : Vec<IpAddr>= vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string())
//     ];

//     // Comparing two enums need to derive the PartialEq trait
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success!");
// }
// =================================================================

// =================================================================
// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);

// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);

// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }

// fn main() {
//     // FILL in the blank
//     let v: Vec<Box<dyn IpAddr>>= vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }
// =================================================================

// =================================================================

// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 1);
//     scores.insert("Katie", 58);

//     // Get returns an Option<&V>
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(98).as_ref());

//     if scores.contains_key("Daniel") {
//         // Indexing returns a value V
//         let score = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score);
//     }
// }
// =================================================================

// =================================================================

// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1: HashMap<&str, i32> = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }
//     println!("Value {:?}", teams_map1);

//     // IMPLEMENT team_map2 in two ways
//     // Tips: one of the approaches is to use `collect` method
//     // let teams_map2 = teams.into_iter().collect();
//     let teams_map2 = teams.iter().cloned().collect();
//     assert_eq!(teams_map1, teams_map2);
//     println!("Success!");
// }
// =================================================================

// FILL in the blanks
// use std::collections::HashMap;
// fn main() {

//     let mut player_stats = HashMap::new();

//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100);

//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);
// }

// fn random_stat_buff() -> u8 {
//     // Could actually return some random value here - let's just return
//     // some fixed value for now
//     42
// }
// =================================================================

// =================================================================

// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits
// use std::collections::HashMap;

// #[derive(Debug)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     /// Creates a new Viking.
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     // Use a HashMap to store the vikings' health points.
//     let mut vikings = vec![
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ];

//     // (Viking::new("Einar", "Norway"), 25),
//     //     (Viking::new("Olaf", "Denmark"), 24),
//     //     (Viking::new("Harald", "Iceland"), 12),

//     // Use derived implementation to print the status of the vikings.
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }
// =================================================================

// =================================================================

// use std::collections::HashMap;
// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::with_capacity(1);
//     map.insert(1, 2);
//     map.insert(3, 4);
//     assert!(map.capacity() <= 100, "Error by {}", map.capacity());

//     map.shrink_to(50);
//     assert!(map.capacity() <= 50, "Error by {}", map.capacity());

//     println!("Value is {}", map.capacity());

//     map.shrink_to_fit();
//     assert!(map.capacity() >= 2);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// FIX the errors and FILL in the blank
// DON'T remove any code
// FIX the errors and FILL in the blank
// DON'T remove any code
// fn main() {
//     let decimal = 97.123_f32;

//     let integer: u8 = decimal as u8;

//     let c1: char = decimal as u8 as char;
//     let c2 = integer as char;

//     assert_eq!(integer, 'a' as u8, "Value is {integer}");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     assert_eq!(u8::MAX, 255);
//     // The max of `u8` is 255 as shown above.
//     // so the below code will cause an overflow error: literal out of range for `u8`.
//     // PLEASE looking for clues within compile errors to FIX it.
//     // DON'T modify any code in main.
//     let v = 1000.0 as u8;

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     assert_eq!(1000 as u16, 1000);

//     assert_eq!(100 as u8, 100);

//     assert_eq!(-1_i8 as u8, 255);

//     // Since Rust 1.45, the `as` keyword performs a *saturating cast*
//     // when casting from float to int. If the floating point value exceeds
//     // the upper bound or is less than the lower bound, the returned value
//     // will be equal to the bound crossed.
//     assert_eq!(300.1_f32 as u8, 255);
//     assert_eq!(-100.1_f32 as u8,0);

//     // This behavior incurs a small runtime cost and can be avoided
//     // with unsafe methods, however the results might overflow and
//     // return **unsound values**. Use these methods wisely:
//     unsafe {
//         println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
//         println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
//         println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
//     }
// }
// =================================================================

// =================================================================

// FILL in the blanks
// fn main() {
//     let mut values: [i32; 2] = [1, 2];
//     let p1: *mut i32 = values.as_mut_ptr();
//     println!("{:?}", p1);
//     let first_address: usize = p1 as usize;
//     let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
//     let p2: *mut i32 = second_address as *mut i32; // p2 points to the 2nd element in values
//     unsafe {
//         // Add one to the second element
//         *p2 += 1
//     }

//     assert_eq!(values[1], 3);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let arr :[u64; 13] = [1; 13];
//     assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
//     let a: *const [u64] = &arr;
//     let b = a as *const [u8];
//     unsafe {
//         assert_eq!(std::mem::size_of_val(&*b), 13)
//     }

//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let my_str = "hello";

//     // three conversions below all depends on the fact: String implements From<&str>:
//     let string1 = String::from(my_str);
//     let string2 = my_str.to_string();
//     // Explicit type annotation is required here
//     let string3: String = my_str.into();
// }
// =================================================================

// =================================================================
// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);
//     let i3: i32 = 'a' as i32;

//     let s: String = String::from('a');

//     println!("Success!");
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(value: i32) -> Self {
//         Number { value }
//     }
// }

// // FILL in the blanks
// fn main() {
//     let num = Number::from(30);
//     assert_eq!(num.value, 30);

//     let num: Number = Number{value: 3};
//     assert_eq!(num.value, 30);

//     println!("Success!");
// }
// =================================================================

// =================================================================
// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     fn from(error: io::Error) -> Self {
//         let error = CliError::IoError(error);
//         return error;
//     }
// }

// impl From<num::ParseIntError> for CliError {
//     fn from(error: num::ParseIntError) -> Self {
//         let error = CliError::ParseError(error);
//         return error;
//     }
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? automatically converts io::Error to CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

// fn main() {
//     println!("Success!");
// }
// =================================================================

// =================================================================
// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;

// fn main() {
//     let n: i16 = 255;

//     let n: u8 = match n.try_into() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0
//         }
//     };

//     assert_eq!(n, 255);

//     println!("Success!");
// }

// =================================================================

// =================================================================
// #[derive(Debug, PartialEq)]
// struct EvenNum(i32);

// impl TryFrom<i32> for EvenNum {
//     type Error = ();

//     // IMPLEMENT `try_from`
//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNum(value))
//         } else {
//             Err(())
//         }
//     }
// }

// fn main() {
//     assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
//     assert_eq!(EvenNum::try_from(5), Err(()));

//     // FILL in the blanks
//     let result: Result<EvenNum, ()> = 8i32.try_into();
//     print!("result {:?}", result);
//     // assert_eq!(result, 0);
//     // let result: Result<EvenNum, ()> = 5i32.try_into();
//     // assert_eq!(result, 1);

//     // println!("Success!");
// }
// =================================================================

// =================================================================
// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "The point is ({}, {})", self.x, self.y)
//     }
// }

// fn main() {
//     let origin = Point { x: 0, y: 0 };
//     // FILL in the blanks
//     assert_eq!(format!("{}", origin), "The point is (0, 0)");
//     assert_eq!(origin.to_string(), "The point is (0, 0)");

//     println!("Success!");
// }
// =================================================================

// =================================================================
// To use `from_str` method, you need to introduce this trait into the current scope.
// use std::str::FromStr;
// fn main() {
//     let parsed: i32 = "5".parse().unwrap();
//     let turbo_parsed = i32::from_str("10").unwrap();
//     let from_str = "20".parse::<i32>().unwrap();
//     let sum = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// FILL the blanks
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // IMPLEMENT the below code
//         panic!("error with {beverage}")
//     }
//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink("Vo Le Hong Vo");
//     println!("Exercise Failed if printing out this line!");
// }
// =================================================================

// =================================================================
// fn divide(x: u8, y: u8) {
//     if y == 0 {
//         println!("Error: Division by zero!");
//     } else {
//         println!("{}", x / y);
//     }
// }

// fn production_rate_per_hour(speed: u8) -> f64 {
//     let cph: u8 = 221;
//     let return_value = (speed as u16 * cph as u16);
//     let value = match speed {
//         1..=4 => return_value as f64,
//         5..=8 => return_value as f64 * 0.9,
//         9..=10 => return_value as f64 * 0.77,
//         _ => 0 as f64,
//     };
//     return value;
// }

// pub fn working_items_per_minute(speed: u8) -> u32 {
//     (production_rate_per_hour(speed) / 60 as f64) as u32
// }
// fn main() {
//     assert_eq!("abc".as_bytes(), [97, 98, 99]);

//     let v = vec![1, 2, 3];
//     let ele = v[2];
//     let ele = v.get(2).unwrap();

//     let v = production_rate_per_hour(2);
//     println!("Value is {:?}", v);

//     divide(15, 0);

//     println!("Success!")
// }
// =================================================================

// =================================================================

// FILL in the blanks and FIX the errors
// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1 = n1_str.parse::<i32>();
//     let n2 = n2_str.parse::<i32>();
//     match (n1, n2) {
//         (Ok(value1), Ok(value2)) => Ok(value1 * value2),
//         (Err(err), _) | (_, Err(err)) => Err(err)
//     }
// }

// fn main() {
//     let result = multiply("10", "2").unwrap();
//     assert_eq!(result, 20);

//     let result = multiply("t", "2").unwrap();
//     assert_eq!(result.__, 8);

//     println!("Success!");
// }
// =================================================================

// =================================================================

// IMPLEMENT multiply with ?
// DON'T use unwrap here
// fn multiply(n1_str: &str, n2_str: &str) -> __ {
// }

// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!");
// }
// =================================================================

// =================================================================
// mod restaurants;

// use crate::restaurants::{
//     hosting::{add_to_waitlist, seat_at_table},
//     house::{
//         back_house::average_order,
//         front_house::{point, review},
//     },
//     serving::{complain, serve_order, take_order, take_payment},
// };

// fn main() {
//     add_to_waitlist();
//     seat_at_table();
//     take_order();
//     take_payment();
//     serve_order();
//     complain();
//     average_order();
//     point();
//     review();
// }
// =================================================================

// =================================================================

/* Make it work, only using comments! */
// fn main() {
//     /*todo!();
//     unimplemented!();
//      */
//     assert_eq!(6, 5 + 3 + 2 + 1 )
// }
// =================================================================

// =================================================================
/// The function `add_one` in Rust takes an integer as input and returns that integer incremented by
/// one.
///
/// Arguments:
///
/// * `x`: The parameter `x` is an `i32` type, which stands for a 32-bit signed integer.
///
/// Returns:
///
/// The function `add_one` returns the value of `x` incremented by 1.
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }
// =================================================================

// =================================================================
/// The function `add_two` in Rust takes an `i32` input and returns the input value incremented by 2.
///
/// Arguments:
///
/// * `x`: The parameter `x` in the `add_two` function is of type `i32`, which means it is a 32-bit
/// signed integer.
///
/// Returns:
///
/// The function `add_two` returns the value of `x` incremented by 2.
// pub fn add_two(x: i32) -> i32 {
//     x + 2
// }
// =================================================================

// =================================================================
/// The function `div` in Rust performs integer division and handles divide-by-zero errors.
///
/// Arguments:
///
/// * `a`: i32 - a 32-bit signed integer
/// * `b`: The parameter `b` represents the divisor in the division operation. It is used to divide the
/// value of `a` in the `div` function.
///
/// Returns:
///
/// the result of dividing `a` by `b`, as long as `b` is not equal to 0. If `b` is 0, the function will
/// panic with a "Divide-by-zero error" message.
// pub fn div(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         panic!("Divide-by-zero error");
//     }

//     a / b
// }
// =================================================================

// =================================================================
/// The function `try_div` in Rust performs division of two integers and returns a `Result` with the
/// result or an error message for divide-by-zero.
///
/// Arguments:
///
/// * `a`: i32
/// * `b`: The parameter `b` is the divisor in the division operation.
///
/// Returns:
///
/// The function `try_div` returns a `Result` enum with either an `Ok` variant containing the result of
/// the division `a / b` if `b` is not zero, or an `Err` variant containing a `String` with the message
/// "Divide-by-zero" if `b` is zero.
// pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err(String::from("Divide-by-zero"))
//     } else {
//         Ok(a / b)
//     }
// }
// =================================================================

// =================================================================
// fn main() {
//     println!("{} days", 31);
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
//     println!("{subject} {verb} {object}",
//              object="the lazy dog",
//              subject="the quick brown fox",
//              verb="jumps over");
//     println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
//     println!("{number:>width$}", number=1, width=6);

//     println!("{number:0>width$}", number=1, width=6);
//     println!("My name is {0}, {1} {0}", "Bond", "lele");

//     #[derive(Debug)]
//     struct Structure(i32);
//     println!("This struct `{:?}` won't print...", Structure(3));
// }
// =================================================================

// =================================================================
// fn main() {
//     let s1 = "hello";
//     /* Fill in the blank */
//     let s = format!("{s1}, world!");
//     assert_eq!(s, "hello, world!");
//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     /* Fill in the blanks to make it print:
//     Hello world, I am
//     Sunface!
//     */
//     println!("{value1} {value2} {value3}", value1="hello world, ", value2="I am", value3="Sunface");
//  }
// =================================================================

// =================================================================
// // This structure cannot be printed either with `fmt::Display` or
// // with `fmt::Debug`.
// struct UnPrintable(i32);

// // To make this struct printable with `fmt::Debug`, we can derive the automatic implementations provided by Rust
// #[derive(Debug)]
// struct DebugPrintable(i32);
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct Structure(i32);

// fn main() {
//     // Types in std and Rust have implemented the fmt::Debug trait
//     println!("{} months in a year.", 12);

//     println!("Now {:?} will print!", Structure(3));
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8
// }

// fn main() {
//     let person = Person { name:  "Sunface".to_string(), age: 18 };

//     /* Make it output:
//     Person {
//         name: "Sunface",
//         age: 18,
//     }
//     */
//     println!("{:?}", person);
// }
// =================================================================

// =================================================================

// use std::fmt;

// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);

// impl fmt::Display for Deep{
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let return_value = write!(f, "Value is {:?}", self);
//         return return_value;
//     }
// }

// fn main() {
//     // The problem with `derive` is there is no control over how
//     // the results look. What if I want this to just show a `7`?

//     /* Make it print: Now 7 will print! */
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }
// =================================================================

// =================================================================

/* Make it work*/
// use std::fmt;

// struct Point2D {
//     x: f64,
//     y: f64,
// }

// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Display: {} + {}i", self.x, self.y)
//     }
// }

// impl fmt::Debug for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
//     }
// }

// fn main() {
//     let point = Point2D { x: 3.3, y: 7.2 };
//     assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
//     assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");

//     println!("Success!");
// }
// =================================================================

// =================================================================

/* Make it work */
// use std::fmt;

// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Extract the value using tuple indexing,
//         // and create a reference to `vec`.
//         let vec = &self.0;

//         write!(f, "[")?;

//         // Iterate over `v` in `vec` while enumerating the iteration
//         // count in `count`.
//         for (count, v) in vec.iter().enumerate() {
//             // For every element except the first, add a comma.
//             // Use the ? operator to return on errors.
//             if count != 0 { write!(f, ", ")?; }
//             write!(f, "{}: {}", count, v)?;
//         }

//         // Close the opened bracket and return a fmt::Result value.
//         write!(f, "]")
//     }
// }

// fn main() {
//     let v = List(vec![1, 2, 3]);
//     assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
//     println!("Success!");
// }
// =================================================================

// =================================================================
/* Fill in the blanks */
// fn main() {
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
//     let value = format!("{1}{0}",  1, 2);
//     println!("Value is {value}");
//     assert_eq!(format!("{1}{0}", 1, 2), "21");
//     // assert_eq!(format!(__, 1, 2), "2112");
//     // println!("Success!");
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct Example<'a> {
//     name: &'a str,
// }

// fn print_value<'b>(example: Example<'b>) {
//     println!("Data is {:?}", example);
// }

// fn main() {
//     {
//         let string_data = String::from("Hello, Rust!");
//         let example_instance = Example { name: &string_data };
//     }

//     print_value(example_instance);

//     println!("Original string: {}", string_data);
// }

// =================================================================

// =================================================================
// /* Annotate the lifetime of `i` and `borrow2` */
// // Lifetimes are annotated below with lines denoting the creation
// // and destruction of each variable.
// // `i` has the longest lifetime because its scope entirely encloses
// // both `borrow1` and `borrow2`. The duration of `borrow1` compared
// // to `borrow2` is irrelevant since they are disjoint.
// fn main() {
//     let i = 3;
//     {
//         let borrow1 = &i; // `borrow1` lifetime starts. ──┐
//         //                                                │
//         println!("borrow1: {}", borrow1); //              │
//     } // `borrow1 ends. ──────────────────────────────────┘
//     {
//         let borrow2 = &i;

//         println!("borrow2: {}", borrow2);
//     }
// }
// =================================================================

// =================================================================
// fn main() {
//     {
//         let x = 5;

//         let r = x;

//         println!("r: {}", r);
//         println!("r: {}", x);
//     }
// }
// =================================================================

// =================================================================
 /* Annotate `r` and `x` as above, and explain why this code fails to compile, in the lifetime aspect. */

/* Annotate `r` and `x` as above, and explain why this code fails to compile, in the lifetime aspect. */

// fn main<'a>() {
//     {
//         let r: &'a i32;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
// }

// =================================================================

// =================================================================
// fn print_one<'a>(x: &'a i32) {
//     println!("`print_one`: x is {}", x);
// }

// fn add_one<'a>(x: &'a mut i32) {
//     *x += 1;
// }

// fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("`print_multi`: x is {}, y is {}", x, y);
// }
// fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn main() {
//     let x = 7;
//     let y = 9;

//     let z;
//     {
//         let invalid_ref = pass_x(&x, &y);
//         z = invalid_ref; // invalid_ref trỏ đến vùng nhớ đã bị giải phóng khi ra khỏi phạm vi này.
//     }

//     print_one(z); // Lỗi: z trỏ đến vùng nhớ không hợp lệ.

//     let mut t = 3;
//     add_one(&mut t);
//     print_one(&t);
// }

// =================================================================

// =================================================================
/* Make it work by adding proper lifetime annotation */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("Hello");
//     let mut result;
//     {
//         let string2 = String::from("Rust");
//         result = longest(&string1, &string2);
//         println!("The longest string is: {}", result);
//     }
//     // println!("The longest string is: {}", result);
// }
// =================================================================

// =================================================================
// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
// fn invalid_output<'a>() -> &'a str {
//     "foo"
// }

// fn main() {
//     invalid_output()
// }
// =================================================================

// =================================================================
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// fn failed_borrow<'a>() {
//     let _x = 12;

//     let y: &'a i32 = &42;
// }

// fn main() {
//     let (four, nine) = (4, 9);

//     print_refs(&four, &nine);

//     failed_borrow();
// }
// =================================================================

// =================================================================

// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// #[derive(Debug)]
// struct NamedBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x = 18;
//     let y = 15;

//     let single = Borrowed(&x);
//     let double = NamedBorrowed { x: &x, y: &y };
//     let reference = Either::Ref(&x);
//     let number    = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);
// }
// =================================================================

// =================================================================
/* Make it work */

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// fn main() {
//     let var_a = 35;
//     let example: Example<'static, 'static>;

//     {
//         let var_b = NoCopyType {};

//         // Sửa đổi lifetime của 'example' và 'var_b' để làm cho chúng khớp nhau
//         example = Example { a: &var_a, b: &var_b }; // this line
//     }

//     println!("(Success!) {:?}", example);
// }

// =================================================================

// =================================================================
/* Make it work */
// /// The above Rust code defines a struct `Example` with two fields `a` and `b`. The `a` field is a
/// reference to an `u32` value, and the `b` field is a reference to a `NoCopyType` struct. However,
/// the `NoCopyType` struct is commented out, so it is not being used in the code.
// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// fn main() {
//     let var_a = 35;
//     let var_b = NoCopyType {};

//     let example: Example<'static, '_> = Example {
//         a: &var_a,
//         b: &var_b,
//     };

//     println!("(Success!) {:?}", example);
// }

// =================================================================

// =================================================================
/* Make it work */

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// fn main()
// {
//   /* 'a tied to fn-main stackframe */
//   let var_a = 35;
//   let var_b = NoCopyType {};

//   let example: Example;

//   {
//     /* Lifetime 'b tied to new stackframe/scope */
//     /* fixme */
//     example = Example { a: &var_a, b: &var_b };
//   }

//   println!("(Success!) {:?}", example);
// }
// =================================================================

// =================================================================

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// /* Fix function signature */
// fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
// { foo.b }

// fn main()
// {
//     let no_copy = NoCopyType {};
//     let example = Example { a: &1, b: &no_copy };
//     let value = fix_me(&example);
//     println!("Value is {:?}", value);
//     println!("Success!")
// }
// =================================================================

// =================================================================
// struct Owner(i32);

// impl Owner {
//     fn add_one<'a>(&'a mut self) { self.0 += 1; }
//     fn print<'a>(&'a self) {
//         println!("`print`: {}", self.0);
//     }
// }

// fn main() {
//     let mut owner = Owner(18);
//     let value: i32 = 10; // line 1
//     {
//         let new_value = Owner(value);// line 2
//     }

//     owner.add_one();
//     owner.print();
// }
// =================================================================

// =================================================================
/* Make it work by adding proper lifetime annotations */
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&'a self) -> i32 {
//         3
//     }
// }

// fn main() {
//     let value = "hello world";
//     let importantExcerpt = ImportantExcerpt{part: &value};
//     let level = importantExcerpt.level();
//     println!("Value is {:?}", level);
// }
// =================================================================

// =================================================================
/* Remove all the lifetimes that can be elided */

// fn input<'a>(x: &'a i32) {
//     println!("`annotated_input`: {}", x);
// }

// fn pass<'a>(x: &'a i32) -> &'a i32 { x }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// }

// struct Owner(i32);

// impl Owner {
//     // Annotate lifetimes as in a standalone function.
//     fn add_one<'a>(&'a mut self) { self.0 += 1; }
//     fn print<'a>(&'a self) {
//         println!("`print`: {}", self.0);
//     }
// }

// struct Person<'a> {
//     age: u8,
//     name: &'a str,
// }

// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {}
// =================================================================

// =================================================================
// use std::fmt::Debug;

// #[derive(Debug)]
// struct Ref<'a, T: 'a>(&'a T);

// fn print<T>(t: T) where T: Debug {
//     println!("`print`: t is {:?}", t);
// }

// fn print_ref<'a, T>(t: &'a T) where
//     T: Debug + 'a {
//     println!("`print_ref`: t is {:?}", t);
// }

// fn main() {
//     let x = 7;
//     let ref_x = Ref(&x);

//     print_ref(&ref_x);
//     print(ref_x);
// }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct DoubleRef<'a, T: 'a> {
//     r: &'a T,
//     s: &'a T
// }
// fn main() {
//     let value = DoubleRef{ r: &1, s: &1};
//     println!("Success! is value is {:?}", value);
// }

// =================================================================

// =================================================================
/* Adding trait bounds to make it work */
// /* Adding trait bounds to make it work */
// fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) 
// where 
//     'a: 'b,
// {
//     y = x;
//     let r: &'b &'a i32 = &&0;
// }

// fn main() {
//     println!("Success!")
// }
// =================================================================

// =================================================================
/* Adding HRTB to make it work!*/
// fn call_on_ref_zero<F>(f: F) where F: for<'a> Fn(&'a i32) {
//     let zero = 0;
//     f(&zero);
// }

// fn main() {
//     println!("Success!");
// }
// =================================================================

// =================================================================
// fn main() {
//     let mut s = String::from("hello");
 
//      let r1 = &s;
//      let r2 = &s;
//      println!("{} and {}", r1, r2);
 
//      let r3 = &mut s;
//      println!("{}", r3);
//  }
// =================================================================

// =================================================================
// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     fn move_to(&mut self, x: i32, y: i32) {
//         self.x = x;
//         self.y = y;
//     }
// }

// fn main() {
//     let mut p = Point { x: 0, y: 0 };
//     let r = &mut p;
//     // Here comes the reborrow
//     let rr: &Point = &*r;

//     println!("{:?}", rr); // Reborrow ends here, NLL introduced

//     // Reborrow is over, we can continue using `r` now
//     r.move_to(10, 10);
//     println!("{:?}", r);
// }
// =================================================================

// =================================================================
/* Make it work by reordering some code */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;
    println!("{:?}", ref2); // Reborrow ends here, NLL introduced

    *ref1 += 1;
    // *ref2 += 2;

    println!("{}", data);
}
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================
// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================
// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================
// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================

// =================================================================
// =================================================================
