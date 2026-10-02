use std::{collections::HashMap, vec};

fn main() {
    // vectors
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);
    println!("{:?}",vec);

    let vec2 = vec![2,3,4,4,5];
    println!("{:?}", vec2);


    // hashmaps
    let mut hash_map = HashMap :: new();
    hash_map.insert(String::from("utkarsh"), 20);
    hash_map.insert(String::from("Raj"), 21);
    println!("{:?}",hash_map);

    println!("{}", hash_map["utkarsh"]);
    let user = hash_map.get("Raj");

    match user {
        Some(v) => println!("{}",v),
        None => println!("No user exists")
    }



    // iterators
    let mut vec3 = vec![1,2,3,4,5,6];
    let mut vec3_iter = vec3.iter();


    let iter3 = vec3.iter();
    let iter4 = iter3.map(|x| x+1);

    for val in iter4 {
        print!("{} ", val);
    }
    println!();

    while let Some(val) = vec3_iter.next() {
        println!("{}", val);
    }

    // this loop will print nothing because the iterator is exhausted by the while loop
    for val in vec3_iter{
        println!("{}", val);
    }

    let vec3_iter_mut = vec3.iter_mut();
    for val in vec3_iter_mut{
        *val = *val * 2;
    }

    println!("{:?}", vec3);


    // iter and iter_mut borrow the data, now we will see into_iter, it basically takes ownership of the collection

    let vec3_into_iter = vec3.into_iter();
    for val in vec3_into_iter{
        println!("{}", val);
    }
    // vec3 is invalid now
    let vec4 = vec![1,2,3,4,5,6,7,8];
    let vec_a = iter_prac(&vec4);

    println!("{:?}", vec_a);
}



fn iter_prac(v : &Vec<i32>) -> Vec<i32>{
    let iter = v.iter();
    let iter2 = iter.filter(|x| *x % 2 == 1 ).map(|x| x * 2);
    // let mut vec_ret: Vec<i32> = Vec::new();

    // for val in iter2 {
    //     vec_ret.push(val);
    // }

    let vec_ret = iter2.collect();

    return vec_ret;
}