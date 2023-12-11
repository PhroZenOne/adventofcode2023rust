use std::time::{Instant};
use crate::day2::Limit;

mod day1;
mod day_1_statics;
mod day2;

fn main() {
    let start = Instant::now();
    println!("day1: {}", day1::parse_file("./data/day1/day_1_input.dat"));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("day1: {}", day2::parse_file("./data/day2/day2.dat",
                                          &Limit { r: 12, g: 13, b: 14 }));
}
