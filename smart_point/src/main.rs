/// The code defines a `Person` struct in Rust, creates instances of `Person` on both the stack and the
/// heap, and prints out their details.
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32
// }

// fn main() {
//     let person_on_stack = Person {
//         name: String::from("ALice"),
//         age: 30
//     };
//     println!("Value on stask: {:?}", person_on_stack);
//     println!("Name of person: {}", person_on_stack.name);
//     println!("Age of person: {}", person_on_stack.age);
//     println!("--------------------------------------------------------");

//     let person_on_heap = Box::new(
//         Person{
//             name: String::from("Vole"),
//             age: 21
//         }
//     );
//     println!("Value on heap: {:?}", person_on_heap);
//     println!("Name of person: {}", (*person_on_heap).name);
//     println!("Age of person: {}", (*person_on_heap).age);
// }

// fn main() {
//     let value = Box::new(5);
//     println!("Value on heap: {:?}", value);
// }

// #[derive(Debug)]
// struct Node {
//     data: i32,
//     left: Option<Box<Node>>,
//     right: Option<Box<Node>>,
// }

// fn new_node(data: i32) -> Node {
//     Node {
//         data,
//         left: None,
//         right: None
//     }
// }

// fn insert(root: Option<Box<Node>>, data: i32) -> Option<Box<Node>> {
//     match root {
//         Some(mut node) => {
//             if data < node.data {
//                 node.left = insert(node.left, data);
//             } else {
//                 node.right = insert(node.right, data);
//             }
//             Some(node)
//         }
//         None => Some(Box::new(new_node(data)))
//     }
// }

// fn in_order_traversal(root: &Option<Box<Node>>) {
//     if let Some(node) = root {
//         in_order_traversal(&node.left);
//         println!("{}", node.data);
//         in_order_traversal(&node.right);
//     }
// }

// fn main() {
//     let mut tree = None;

//     // Thêm các giá trị vào cây
//     let values = vec![5, 3, 7, 2, 4, 6, 8];
//     for value in values {
//         tree = insert(tree, value);
//     }

//     // // In ra cây theo thứ tự in-order
//     in_order_traversal(&tree);
// }

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

// use crate::List::{Cons, Nil};
// fn main () {
//     let value = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
//     println!("Value is {:?}", value);
// }
// #[derive(Debug)]
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// // fn main() {
// //     let x = 5;
// //     let y = MyBox(x);

// //     assert_eq!(5, x);
// //     assert_eq!(5, *y);
// // }

// fn hello(name: &str) {
//     println!("Hello, {name}");
// }

// fn main() {
//     let message = MyBox::new(String::from("Rust"));
//     let value = &(*message.0)[..];
//     hello(value);
// } 

// struct CustomSmartPointer {
//     data: String
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let value1 = CustomSmartPointer {
//         data: String::from("Mt yourself")
//     };

//     let value2 = CustomSmartPointer {
//         data: String::from("other stuff")
//     };

//     println!("CustomSmartPointers created.");
//     println!("--------------------------------")
// }

// use std::rc::Rc;
// use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, Rc<List>),
//     Nil
// }

// fn main () {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
//     println!("Reference count after creating b: {}", Rc::strong_count(&a));
//     println!("Reference count after creating c: {}", Rc::strong_count(&a));
//     {
//         let m = Cons(4, Rc::clone(&a));
//         let m = Cons(4, Rc::clone(&a));
//         let m = Cons(4, Rc::clone(&a));
//         let m = Cons(4, Rc::clone(&a));
//         let m = Cons(4, Rc::clone(&a));
//         let m = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     drop(b);
//     println!("Reference count after creating c: {}", Rc::strong_count(&a));
//     drop(c);
//     println!("Reference count after creating b: {}", Rc::strong_count(&a));
//     drop(a);
// }

// use std::{cell::RefCell, rc::Rc};

// fn main() {
//     let x = RefCell::new(5);

//     let mut mul_ref: std::cell::RefMut<'_, i32> = x.borrow_mut();
//     *mul_ref += 1;

//     let imm_ref = x.borrow();
//     println!("Immutable Reference: {}", *imm_ref)
// }

// fn main() {
//     let x = RefCell::new(42);

//     // Attempt to break borrowing rules
//     let imm_ref = x.borrow();
//     let mut_ref = x.borrow_mut(); // This will panic at runtime
// }

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct EmailMessenger;

// impl Messenger for EmailMessenger {
//     fn send(&self, msg: &str) {
//         println!("Sending message: {}", msg);
//     }
// }

// pub struct LimitTracker<'a, T: Messenger>  {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl <'a, T> LimitTracker<'a, T> where T: Messenger {
//     pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;
//         let percentage_of_value = 
//     } 
// }

// struct MyStruct {
//     data: i32,
//     shared: Rc<RefCell<MyStruct>>
// }

// fn main() {
//     let my_data = Rc::new([10, Rc::new(Nil)]);
// }


use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(.., item) => Some(item),
            Nil => None
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
}