fn fib(num:i32) -> i32{
    let mut fib = 0;
    for i in 0..num+1{
        fib+=i;
    }
    return fib;
}