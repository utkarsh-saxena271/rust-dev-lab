// structs
struct User {
    active:bool,
    name:String
}

fn main(){
    let u1 = User{
        active:true,
        name:String::from("utkarsh")
    };
    print!("{}",u1.name);
}