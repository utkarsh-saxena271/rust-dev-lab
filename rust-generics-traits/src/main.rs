pub trait Introduce {
    fn introduce(&self)->String{
        return String::from("Introduce");
    }
}

pub trait SomethingElse{
    fn something_else(&self){
        println!("Something else")
    }
}

struct User {
    name : String,
    age : i32
}

impl SomethingElse for User{}

impl Introduce for User {
    fn introduce(&self)->String{
        return format!("Hey, my name is {} and I am {} years old", self.name, self.age);
    }
}


// this means input should be the one that is implementing the summary trait and somethingElse trait, both syntaxes are correct, and second is call trait bound with generics
fn notify(item: &(impl Introduce + SomethingElse)) {
    println!("Notified");
}

fn stuff<T:Introduce + SomethingElse>(item : &T){
    println!("Stuff");
}

fn main() {
    let u1 = User{
        name:String::from("Utkarsh"),
        age:20
    };

    let intro = u1.introduce();
    println!("{}", intro);

    notify(&u1);
    stuff(&u1);

    // let lar = largest(2,3);
    // print!("{}", lar);
}















// generic 
// fn largest<T: std::cmp::PartialOrd>(a:T, b:T)->T{
//     if a > b{
//         a
//     }else{
//         b
//     }
// }