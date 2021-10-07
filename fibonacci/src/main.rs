fn main() {
    for num in 1..11 {
        println!("fibonacci ({}) = {}", num, fib(num));
    }
}
fn fib (num: i32) -> i32 {
    if num <= 0 {
        return 0;
    } else if num== 1{
        return 1;
    } else {
        return fib (num-1)  + fib(num-2);
    }
}