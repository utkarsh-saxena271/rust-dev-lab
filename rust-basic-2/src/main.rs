fn main() {

    let a = 1;
    let is_adult = false;

    let arr = [1,2,3,4,5,6];
    let mut vctr = vec![1,2,3,4,5];

    vctr.push(100);

    println!("{:?}", arr);
    println!("{:?}", vctr);

    let greet = String::from("Hello!");

    println!("{}",greet);
    
    if is_adult {
        println!("You are adult");
    }else {
        println!("You are not an adult");
    }
    println!("{}", a);

    let mut ans = 0;
    for i in 0..arr.len(){
        ans += arr[i];
    }
    println!("{}",ans);
}
