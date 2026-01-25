fn main() {
    println!("{}",fib(0));
    println!("{}",fib(1));
    println!("{}",fib(2));
    println!("{}",fib(3));

}
fn fib(num: u32) -> u64 {
    if num == 0 { return 0; }
    if num == 1 { return 1; }

    let mut first = 0;
    let mut second = 1;


    for _ in 1..num {
        let temp = second;
        second = second + first;
        first = temp;
    }
    
    return second;
} 

