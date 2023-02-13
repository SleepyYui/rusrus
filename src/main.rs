extern crate num;
use num::{BigUint, One, Zero};
//use std::ops::Mul;

// 3x+1 function
// return the count and the highest number
fn three_x_plus_one (mut n: u128) -> (u128, u128) {
    let mut count = 0;
    let mut highest = n;
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
}

fn do_3xp1() {
    let input_message = "Enter a number: ";
    let mut input = String::new();
    println!("{}", input_message);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // start timer
    let start = std::time::Instant::now();
    // run the 3x+1 function with the input and print how long it took to reach 1 and what the highest number was
    let (count, highest) = three_x_plus_one(input.trim().parse().expect("Please type a number!"));
    // end timer
    let end = std::time::Instant::now();
    println!("It took {} steps to reach 1 and the highest number was {}", count, highest);
    println!("It took {} microseconds to run.", end.duration_since(start).as_micros());
}

fn main() {
    do_3xp1();
}