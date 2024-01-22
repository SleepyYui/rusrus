extern crate num;
//use std::ops::Mul;

// 3x+1 function
// return the count and the highest number
pub(crate) fn three_x_plus_one (mut n: u128) -> (u128, u128) {
    let mut count = 0;
    let mut highest = 0;
    while n != 1 {
        //println!("{}", n);
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
}
