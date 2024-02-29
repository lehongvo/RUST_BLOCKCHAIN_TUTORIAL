// fn main() {
//     let total_fn = |x, y| x + y;
//     let result = total_fn(10, 1);
//     println!("Value is {:?}", result);
// }

// fn main() {
//     let vector = vec![1, 3, 4, 6, 6];
//     let max_value = vector.iter().max();

//     match max_value {
//         Some(value) => println!("Max value is {:?}", value),
//         None => println!("Error now")
//     }

//     let sum_value: i32 = vector.iter().sum();
//     println!("Sum value is {:?}", sum_value);
// }


// fn main() {
//     let total_fn = |x: i32, y: i32| -> i32{
//         let value = x + y;
//         return value;
//     };

//     let mut_fn = |a: u128| -> u128 {
//         let mut_value = a * a;
//         return mut_value;
//     };

//     println!("Value is {:?}", mut_fn(1123123123123211201))
// }

fn find_min_len(vector: Vec<&str>) {
    for value in vector {
        println!("String {} have length {}", value, value.len());
    }
}

fn main() {
    let string_vector = vec!["hello", "world", "rust", "iterator"];
    find_min_len(string_vector);
}