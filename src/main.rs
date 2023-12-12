use std::time::{Instant};
use crate::day2::Limit;

mod day1;
mod day_1_statics;
mod day2;

fn main() {
    let start = Instant::now();
    println!("day1: {}", day1::parse_file("./data/day1/day_1_input.dat"));
    println!("day2: {}", day2::parse_file_part_1("./data/day2/day2.dat",&Limit { r: 12, g: 13, b: 14 }));
    println!("day2p2: {}", day2::parse_file_part_2("./data/day2/day2.dat"));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
