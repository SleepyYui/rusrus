extern crate num;
use num::{BigUint, One, Zero};
//use std::ops::Mul;

// 3x+1 function
// return the count and the highest number
/*fn three_x_plus_one (mut n: u64) -> (u64, u64) {
    let mut count = 0;
    let mut highest = 0;
    while n != 1 {
        println!("{}", n);
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
        if n > highest {
            highest = n;
        }
    }
    (count, highest)
}*/


/*fn do_3xp1() {
    let input_message = "Enter a number: ";
    let mut input = String::new();
    println!("{}", input_message);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // run the 3x+1 function with the input and print how long it took to reach 1 and what the highest number was
    let (count, highest) = three_x_plus_one(input.trim().parse().expect("Please type a number!"));
    println!("It took {} steps to reach 1 and the highest number was {}", count, highest);
}*/

// calculate the fibonacci sequence
#[derive(Clone)]
struct Matrix {
    m: [[BigUint; 2]; 2],
}

impl Matrix {
    fn mul(self, other: Matrix) -> Matrix {
        let mut result = Matrix { m: [[BigUint::zero(), BigUint::zero()], [BigUint::zero(), BigUint::zero()]] };
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let product = &self.m[i][k] * &other.m[k][j];
                    result.m[i][j] = &result.m[i][j] + product;
                }
            }
        }
        result
    }
}

fn power(base: &Matrix, exponent: u64) -> Matrix {
    let mut result = Matrix { m: [[BigUint::one(), BigUint::zero()], [BigUint::zero(), BigUint::one()]] };
    let mut current = base.clone();
    let mut exp = exponent;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.mul(current.clone());
        }
        current = current.mul(current.clone());
        exp /= 2;
    }
    result
}



fn calculate_fibonacci(n: u64) -> BigUint {
    let base = Matrix { m: [[BigUint::one(), BigUint::one()], [BigUint::one(), BigUint::zero()]] };
    let result = power(&base, n);
    result.m[0][1].clone()
}

fn main() {
    //do_3xp1();
    println!("{}", calculate_fibonacci(100000000));
}