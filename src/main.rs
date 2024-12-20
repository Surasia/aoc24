use day1::{day1_p1, day1_p2};
use day2::{day2_p1, day2_p2};
use day3::{day3_p1, day3_p2};
use day4::{day4_p1, day4_p2};
use day5::{day5_p1, day5_p2};
use day6::{day6_p1, day6_p2};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("--- Day 1 ---");
    day1_p1();
    day1_p2();
    println!("--- Day 2 ---");
    day2_p1();
    day2_p2();
    println!("--- Day 3 ---");
    day3_p1();
    day3_p2();
    println!("--- Day 4 ---");
    day4_p1();
    day4_p2();
    println!("--- Day 5 ---");
    day5_p1();
    day5_p2();
    println!("--- Day 6 ---");
    day6_p1();
    day6_p2();
}
