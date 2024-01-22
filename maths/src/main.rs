mod three_x_plus_one;
mod fib_matriz;

use std::time::Instant;
use three_x_plus_one::three_x_plus_one;
use fib_matriz::fib_matrix;

// Function to get user input
fn get_user_input() -> u128 {
    let input_message: &str = "Enter a number: ";
    let mut input: String = String::new();
    println!("{}", input_message);
    std::io::stdin().read_line(&mut input).unwrap_or_else(|_| panic!("Failed to read line"));
    input.trim().parse().unwrap_or_else(|_| panic!("Please type a number!"))
}

// Function to measure execution time
fn measure_execution_time<F: FnOnce() -> T, T>(f: F) -> (T, u128) {
    let start: Instant = Instant::now();
    let result: T = f();
    let end: Instant = Instant::now();
    (result, end.duration_since(start).as_micros())
}

fn main() {
    let number: u128 = get_user_input();

    // Measure the execution time of the three_x_plus_one function
    let ((count, highest), duration) = measure_execution_time(|| three_x_plus_one(number));
    println!("It took {} steps to reach 1 and the highest number was {}", count, highest);
    println!("It took {} microseconds to run.", duration);

    // Measure the execution time of the fib_matrix function
    let (result, duration) = measure_execution_time(|| fib_matrix(number as usize));
    println!("The {}th Fibonacci number is: ", number);
    let result_str: String = result.to_str_radix(36);
    let result_str: &str = if result_str.len() > 100 {
        result_str.split_at(100).0
    } else {
        &result_str
    };
    println!("{}", result_str);
    println!("It took {} microseconds to run.", duration);
}