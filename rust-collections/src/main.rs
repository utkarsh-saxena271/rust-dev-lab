use std::collections::HashMap;

fn main() {
    // vectors
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);
    println!("{:?}",vec);


    // hashmaps
    let mut hash_map = HashMap :: new();
    hash_map.insert(String::from("utkarsh"), 20);
    hash_map.insert(String::from("Raj"), 21);
    println!("{:?}",hash_map);

}