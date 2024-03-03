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

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use crate::List::{Cons, Nil};
fn main () {
    let value = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))));
    println!("Value is {:?}", value);
}