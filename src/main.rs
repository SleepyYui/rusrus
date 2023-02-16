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
        const m: u128 = n / 2;
        // fib_m = fib(m, memo)
        const fib_m: u128 = fib(m, memo);
        // result = fib_m * (fib_m + 2 * fib(m-1, memo))
        const result: u128 = fib_m * (fib_m + fib(m - 1, memo) * BigUint::from(2u8));
        // memo[n] = result
        memo[n as usize] = Some(result.clone());
        // return result
        result
    }
    // if n is odd
    else {
        // m = (n + 1) // 2
        const m: u128 = (n + 1) / 2;
        // fib_m = fib(m, memo)
        const fib_m: u128 = fib(m, memo);
        // fib_m1 = fib(m-1, memo)
        const fib_m1: u128 = fib(m - 1, memo);
        // result = fib_m * fib_m + fib_m1 * fib_m1
        const result: u128 = fib_m * fib_m + fib_m1 * fib_m1;
        // memo[n] = result
        memo[n as usize] = Some(result.clone());
        // return result
        result
    }
}


fn main() {
    fib(1_000_000_000, &mut vec![None; 1_000_000_000 + 1]);
}
