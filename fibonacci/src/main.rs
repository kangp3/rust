fn main() {
    let n = 6;
    let n_fib = fibonacci(n);
    println!("{n} Fibonacci number is: {n_fib}");
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }
    fibonacci(n-1) + fibonacci(n-2)
}
