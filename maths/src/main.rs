// import functions from other 3xp1 and fib_matriz files

mod three_x_plus_one;
mod fib_matriz;

use three_x_plus_one::three_x_plus_one;
use fib_matriz::fib_matrix;

fn main() {
    let input_message = "Enter a number: ";
    let mut input = String::new();
    println!("{}", input_message);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let n = input.trim().parse().expect("Please type a number!");

    // start timer
    let mut start = std::time::Instant::now();
    // run the 3x+1 function with the input and print how long it took to reach 1 and what the highest number was
    let (count, highest) = three_x_plus_one(n);
    // end timer
    let mut end = std::time::Instant::now();
    println!("It took {} steps to reach 1 and the highest number was {}", count, highest);
    println!("It took {} microseconds to run.", end.duration_since(start).as_micros());

    start = std::time::Instant::now();
    let result = fib_matrix(n.try_into().unwrap());
    end = std::time::Instant::now();

    println!("The {}th Fibonacci number is: ", n);
    println!("{}...", result.to_str_radix(36).split_at(100).0);
    println!("It took {} microseconds to run.", end.duration_since(start).as_micros());
}