fn main() {
    let fib_number = 12;
    let result = fibonacci(fib_number); // time complexity = O(n)

    println!("{result}");
}

fn fibonacci(n: u64) -> u64 {
    match n {
        // handle base cases
        0 => return 0,
        1 => return 1,

        // match every other case
        _ => {
            // using f(n) = f(n - 1) + f(n - 2)
            let mut f0 = 0;
            let mut f1 = 1;
            let mut fib = 0;

            for _ in 2..=n {
                fib = f0 + f1;
                f0 = f1;
                f1 = fib;
            }

            return fib;
        }
    }
}
