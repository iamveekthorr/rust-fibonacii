fn main() {
    // using f(n) = f(n - 1) + f(n - 2)
    let fib_number = 12;
    let result = fibonacci(fib_number); // time complexity = O(n)

    let res = fibonacci_2(fib_number as usize);

    println!("{result}");
    println!("{res}");
}

fn fibonacci(n: u64) -> u64 {
    match n {
        // handle base cases
        0 => 0,
        1 => 1,

        // match every other case
        _ => {
            let mut f0 = 0;
            let mut f1 = 1;
            let mut fib = 0;

            for _ in 2..=n {
                fib = f0 + f1;
                f0 = f1;
                f1 = fib;
            }

            fib
        }
    }
}

fn fibonacci_2(n: usize) -> u32 {
    let mut ans = vec![0, 1];

    for i in 2..=n {
        ans.push(ans[i - 1] + ans[i - 2]);
    }

    ans[n]
}
