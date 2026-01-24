// structs
// struct User {
//     active:bool,
//     name:String
// }

// fn main(){
//     let u1 = User{
//         active:true,
//         name:String::from("utkarsh")
//     };
//     print!("{}",u1.name);
// }


// Mutable references

fn main() {
    let mut s1 = String::from("Hello");
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}