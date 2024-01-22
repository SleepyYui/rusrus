use num::{BigUint, One, Zero};
//use num::ToPrimitive;

pub(crate) fn fib_matrix(n: usize) -> BigUint {
    let mut f = vec![vec![BigUint::one(), BigUint::one()],
                     vec![BigUint::one(), BigUint::zero()]];
    let mut res = vec![vec![BigUint::one(), BigUint::zero()],
                       vec![BigUint::zero(), BigUint::zero()]];

    let mut n = n;
    while n > 0 {
        //println!("{} iterations remaining.", n);
        if n & 1 == 1 {
            res = mat_mul(&res, &f);
        }
        f = mat_mul(&f, &f);
        n >>= 1;
    }
    res[0][1].clone()
}

fn mat_mul(a: &Vec<Vec<BigUint>>, b: &Vec<Vec<BigUint>>) -> Vec<Vec<BigUint>> {
    let mut res = vec![vec![BigUint::zero(), BigUint::zero()],
                       vec![BigUint::zero(), BigUint::zero()]];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                //println!("{} {} {}", i, j, k);
                res[i][j] += &a[i][k] * &b[k][j];
                //println!("calculated successfully.");
            }
        }
    }
    res
}

fn main() {
    let n = 1_000_000_000;
    // get n from the user
    /*let input_message = "Enter a number: ";
    let mut input = String::new();
    println!("{}", input_message);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n = input.trim().parse().expect("Please type a number!");*/
    /*let result = */fib_matrix(n);
    println!("The {}th Fibonacci number is: ", n);
    //println!("{}", result.to_str_radix(36));
}
