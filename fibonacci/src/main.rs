fn main() {
    let n = 41;
    let n_fib = fibonacci(n);
    println!("{n} Fibonacci number is: {n_fib}");
    let n_fib = fibonacci_recursive(n);
    println!("{n} Fibonacci number is: {n_fib}");
}

fn fibonacci(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut prev: u64 = 1;
    let mut result: u64 = 1;
    for _ in 0..n-2 {
        let tmp = result;
        result = result + prev;
        prev = tmp;
    }
    return result;
}

fn fibonacci_recursive(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    fibonacci_recursive(n-2) + fibonacci_recursive(n-1)
}
