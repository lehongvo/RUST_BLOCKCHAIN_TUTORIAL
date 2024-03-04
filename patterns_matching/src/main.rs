// fn main() {
//     // let x = 5;
//     // match x {
//     //     1 => println!("One"),
//     //     2 => println!("Two"),
//     //     _ => println!("Some value we can not understand"),
//     // }

//     // let value = (1, "Hello, world!");
//     // match value {
//     //     (1, "Hello, world") => println!("OK value"),
//     //     (2, "Hello, world!") => println!("Not ok value"),
//     //     (_) => println!("Some value we can not understand"),
//     // }

//     // let point = (3, 5);
//     // match point {
//     //     (x, 0) => println!("Trên trục x tại {}", x),
//     //     (0, y) => println!("Trên trục y tại {}", y),
//     //     (3, 5) => println!("Not ok value"),
//     //     _ => println!("Không nằm trên bất kỳ trục nào"),
//     // }

//     // let some_value = Some(16);
//     // match some_value {
//     //     Some(value) => println!("Value is {}", value),
//     //     None => println!("Error value")
//     // }

//     // let x = 5;
//     // if let Some(y) = Some(x) {
//     //     println!("Value is {}", y);
//     // }

//     // let some_value: Option<i32> = None;
//     // if let Some(value) = some_value {
//     //     println!("Refutable pattern matched! This won't be printed.");
//     // } else {
//     //     println!("Refutable pattern didn't match. Handling the None case.");
//     // }

//     // let favorite_color: Option<&str>= None;
//     // let is_tuesdays = false;
//     // let age: Result<u8, _> = "34".parse();

//     // if let Some(color) = favorite_color {
//     //     println!("Using your favorite color, {color}, as the background");
//     // } else {
//     //     if is_tuesdays {
//     //         println!("Tuesday is green day!");
//     //     } else {
//     //         if let Ok(age) = age {
//     //             if age > 30 {
//     //                 println!("Using purple as the background color");
//     //             } else {
//     //                 println!("Using orange as the background color");
//     //             }
//     //         } else {
//     //             println!("Using blue as the background color");
//     //         }
//     //     }
//     // }

//     // let mut stack = vec![];
//     // stack.push(1);
//     // stack.push(2);
//     // stack.push(3);
//     // while let Some(top) =  stack.pop(){
//     //     println!("{}", top);
//     // }

//     // let vector = vec![1,2,3];
//     // for(index, value) in vector.iter().enumerate() {
//     //     println!("Index: {:?} value: {:?}", index, value);
//     // }    
//     let (x, y, z) = (1, 2, 3);
//     println!("{}, {}, {}", x, y, z);
// }


// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: ({}, {})", x, y);
// }

// fn main() {
//     let point = (3, 5);
//     print_coordinates(&point);
// }


// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg: Message = Message::Write(String::from("Write"));

//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure.");
//         }
//         Message::Move { x, y } => {
//             println!("Move in the x direction {x} and in the y direction {y}");
//         }
//         Message::Write(text) => {
//             println!("Text message: {text}");
//         }
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {r}, green {g}, and blue {b}",)
//         }
//     }
// }
// #[derive(Debug)]
// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("Change color to red {r}, green {g}, and blue {b}");
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("Change color to hue {h}, saturation {s}, value {v}")
//         }
//         _ => (),
//     }
// }

fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}