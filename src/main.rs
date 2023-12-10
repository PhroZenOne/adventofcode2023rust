use std::time::{Instant};
mod day1;
mod day_1_statics;

fn main() {
    let start = Instant::now();
    println!("day1: {}", day1::parse_file("./data/day1/day_1_input.dat"));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
