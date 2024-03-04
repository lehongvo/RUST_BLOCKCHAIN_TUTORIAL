// use std::thread;

// fn main() {
//     let handle = thread::spawn(move || {
//         println!("You touched me");
//     });
//     handle.join().unwrap();
//     println!("Our of the following");
// }

// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     let (sender, receiver) = mpsc::channel::<String>();
//     let (sende1, receiver1) = mpsc::channel::<String>();
//     let handle = thread::spawn(move || {
//         sender.send(String::from("Hello from a thread!"));
//         sende1.send(String::from("Iam a monster!"));
//     });
//     let received_message= receiver.recv().unwrap();
//     let received_message1 = receiver1.recv().unwrap();
//     println!("Received message: {}", received_message);
//     println!("received_message: {}", received_message1);
//     handle.join().unwrap();
// }

// use std::{sync::{Arc, Mutex}, thread};

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut number = counter.lock().unwrap();
//             *number += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// use std::{thread, time::Duration};

// fn main() {
//     let new_thread =  thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     new_thread.join().unwrap();

//     for i in 1..5 {
//         println!("hi number--- {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// use std::thread;
// fn main() {
//     let v = vec![1,2,3];
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });
//     handle.join().unwrap();
// }

// use std::{sync::mpsc, thread, time::Duration};

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

use std::{sync::{Mutex, Arc}, thread, rc::Rc};

// fn main() {
//     let m = Mutex::new(6);
//     {
//         let mut num = m.lock().unwrap();
//         *num +=1;
//         println!("Value is {}", *num);
//     }
//     println!("Value: {:?}", m);
// }

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_copy = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_copy.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}