#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/* Advent of Code - Day 2 - Surasia */

const PATH_TO_INPUT: &str = "files/day2.txt";
const MAX_ABS_VAL: i32 = 3;
const MIN_ABS_VAL: i32 = 1;

pub fn day2_p1() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let mut valid_lines = 0;
    for line in br.lines().map_while(Result::ok) {
        let inputs = line
            .split_whitespace() // okay im finally using this function
            .map(|s| s.parse::<i32>().expect("Could not convert value to i32!"))
            .collect::<Vec<_>>();
        if check_nums(&inputs[..]) {
            valid_lines += 1;
        }
    }
    println!("Day 2 safe reports: {valid_lines}");
}

fn check_nums(inputs: &[i32]) -> bool {
    let mut valid = true;
    // We just need to check if its increasing now
    let mut decreasing = true;
    for i in 1..inputs.len() {
        let diff = inputs[i] - inputs[i - 1];
        if (diff.abs() > MAX_ABS_VAL) | (diff.abs() < MIN_ABS_VAL) {
            valid = false;
            break;
        }

        if diff >= 1 && i == 1 {
            // is it negative? we check first - second ONLY
            decreasing = false;
        }

        if diff < 0 && !decreasing && i != 1 {
            // number decreased but we are increasing
            valid = false;
            break;
        }
        if diff > 0 && decreasing && i != 1 {
            // number increased but we are decreasing
            valid = false;
            break;
        }
    }
    valid
}

pub fn day2_p2() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let mut valid_lines = 0;
    for line in br.lines().map_while(Result::ok) {
        let mut tolerated = false; // If ANY value is removed and it works
        let inputs = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Could not convert value to i32!"))
            .collect::<Vec<_>>();
        let initial = check_nums(&inputs[..]); // basic check for initial first
        if initial {
            valid_lines += 1;
            continue;
        }
        for perm in 0..inputs.len() {
            let mut slice = inputs.clone(); // make a unique input vec each time..
            slice.remove(perm); // we want to get rid of only a single value and check ALL
            let check = check_nums(&slice[..]);
            if check {
                tolerated = true;
                break;
            }
        }

        if tolerated {
            valid_lines += 1;
        }
    }
    println!("Day 2 safe dampened reports: {valid_lines}");
}
