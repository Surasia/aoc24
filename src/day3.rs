#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use std::{
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

/* Advent of Code - Day 3 - Surasia */

const PATH_TO_INPUT: &str = "files/day3.txt";

// Matches mul(x, y) [0, 1, 2]
const REGEX_MULTIPLY: &str = r"mul\((\d{1,3}),(\d{1, 3})\)";

// Matches do() [0] and don't() [0] OR mul(x, y) [1, 2, 3]
const REGEX_MULTIPLY_CONDITIONAL: &str = r"(do\(\)|don't\(\))|mul\((\d{1,3}),(\d{1, 3})\)";

pub fn day3_p1() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let mut br = BufReader::new(file);
    let re = Regex::new(REGEX_MULTIPLY).expect("Failed to parse regex!");
    let mut file_buffer = String::new();
    br.read_to_string(&mut file_buffer)
        .expect("Failed to read to string!");

    let sum = re
        .captures_iter(&file_buffer)
        .map(|c| {
            c[1].parse::<u32>().expect("failed to convert to u32!")
                * c[2].parse::<u32>().expect("failed to convert to u32!")
        })
        .sum::<u32>();
    println!("Day 3 mul instruction sum: {sum}");
}

pub fn day3_p2() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let mut br = BufReader::new(file);
    let re = Regex::new(REGEX_MULTIPLY_CONDITIONAL).expect("Failed to parse regex!");
    let mut file_buffer = String::with_capacity(br.buffer().len());
    br.read_to_string(&mut file_buffer)
        .expect("Failed to read to string!");

    let mut should_do = true;
    let sum = re
        .captures_iter(&file_buffer)
        .filter_map(|c| match &c[0] {
            "do()" => {
                should_do = true;
                None
            }
            "don't()" => {
                should_do = false;
                None
            }
            _ if !should_do => None,
            _ => match (c[2].parse::<u32>(), c[3].parse::<u32>()) {
                (Ok(n1), Ok(n2)) => Some(n1 * n2),
                _ => None,
            },
        })
        .sum::<u32>();
    println!("Day 3 conditional mul instruction sum: {sum}");
}
