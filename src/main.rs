use num::{BigUint, One};

fn fib(n: u64, memo: &mut Vec<Option<BigUint>>) -> BigUint {
    // if n in memo, return memo[n]
    if let Some(x) = memo[n as usize] {
        return x;
    }
    // if n <= 2, return 1
    if n <= 2 {
        return BigUint::one();
    }
    // if n is even
    if n % 2 == 0 {
        // m = n // 2
        let m = n / 2;
        // fib_m = fib(m, memo)
        let fib_m = fib(m, memo);
        // result = fib_m * (fib_m + 2 * fib(m-1, memo))
        let result = fib_m * (fib_m + fib(m - 1, memo) * BigUint::from(2u8));
        // memo[n] = result
        memo[n as usize] = Some(result.clone());
        // return result
        result
    }
    // if n is odd
    else {
        // m = (n + 1) // 2
        let m = (n + 1) / 2;
        // fib_m = fib(m, memo)
        let fib_m = fib(m, memo);
        // fib_m1 = fib(m-1, memo)
        let fib_m1 = fib(m - 1, memo);
        // result = fib_m * fib_m + fib_m1 * fib_m1
        let result = fib_m * fib_m + fib_m1 * fib_m1;
        // memo[n] = result
        memo[n as usize] = Some(result.clone());
        // return result
        result
    }
}


fn main() {
    fib(1_000_000_000, &mut vec![None; 1_000_000_000 + 1]);
}
