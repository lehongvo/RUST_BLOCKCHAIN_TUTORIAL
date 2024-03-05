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
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1; 
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    println!("Value is {count}");

    assert!(count == __);

    println!("Success!");
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
