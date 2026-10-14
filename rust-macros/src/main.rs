use std::fmt::{
    self,
    Display,
};


#[derive(Debug)]
struct Rect {
    l : i32,
    b : i32
}

impl Display for Rect{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length is {} andcd breadth is {}",self.l, self.b)
    }
}

impl Rect {
    fn area(&self) -> i32 {
        self.l * self.b
    }

    fn shape() -> String {
        return String::from("Rectangle")
    }
}


fn main(){
    let r1 = Rect{
        l : 10,
        b : 12
    };

    println!("{}", r1.area());
    println!("{}", Rect::shape());
    println!("{}", r1);
    println!("{:?}", r1);
}