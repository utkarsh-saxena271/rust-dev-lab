struct User {
    name : String,
    active: bool,
    age: i32
}

impl User{
    fn display_name(&self){
        println!("{}",self.name);
    }
}


fn main(){
    let user1 = User{
        name:String::from("utkarsh"),
        active:true,
        age:20
    };

    // println!("{}", user1.name);
    user1.display_name();
    println!("{}", user1.age);
    println!("{}", user1.active);
}