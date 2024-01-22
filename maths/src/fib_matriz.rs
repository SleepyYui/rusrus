use num::{BigUint, One, Zero};
use rayon::prelude::*;

pub(crate) fn fib_matrix(n: usize) -> BigUint {
    let mut f = vec![vec![BigUint::one(), BigUint::one()],
                     vec![BigUint::one(), BigUint::zero()]];
    let mut res = vec![vec![BigUint::one(), BigUint::zero()],
                       vec![BigUint::zero(), BigUint::zero()]];

    let mut n = n;
    while n > 0 {
        println!("{} iterations remaining.", n);
        if n & 1 == 1 {
            res = mat_mul(&res, &f);
        }
        f = mat_mul(&f, &f);
        n >>= 1;
    }
    res[0][1].clone()
}

use std::sync::Mutex;

fn mat_mul(a: &Vec<Vec<BigUint>>, b: &Vec<Vec<BigUint>>) -> Vec<Vec<BigUint>> {
    let res = Mutex::new(vec![vec![BigUint::zero(); 2]; 2]);
    (0..2).into_par_iter().for_each(|i| {
        for j in 0..2 {
            for k in 0..2 {
                let mut res = res.lock().unwrap();
                let mut res_ij = res[i][j].clone();
                res_ij += &a[i][k] * &b[k][j];
                res[i][j] = res_ij;
            }
        }
    });
    res.into_inner().unwrap()
}