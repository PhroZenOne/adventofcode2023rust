use std::time::{Instant};
mod runner;
mod statics;

fn main() {
    let start = Instant::now();
    println!("day1: {}",  runner::parse_file("./data/day_1_input.dat"));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
