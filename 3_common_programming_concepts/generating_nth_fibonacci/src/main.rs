use std::io;
fn main() {
    println!("Please input your nth fibonacci number");

    let mut nthFibonacci = String::new();

    io::stdin()
        .read_line(&mut nthFibonacci)
        .expect("Failed to read line");

    let nthFibonacci: i32 = nthFibonacci.trim().parse().expect("Expected a number");

    let fiboNumber: i32 = fibonacciNumber(nthFibonacci);

    println!("{nthFibonacci}. number is {fiboNumber}");
}


fn fibonacciNumber(n: i32) -> i32 {
    if n < 0 {
        println!("Invalid Number {n}");
        0
    } else if n == 2 || n==1 {
        1
    } else {
        fibonacciNumber(n-1) + fibonacciNumber(n - 2)
    }
}
