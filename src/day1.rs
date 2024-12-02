#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/* Advent of Code - Day 1 - Surasia */

const WHITESPACE: &str = "   ";
const PATH_TO_INPUT: &str = "files/day1.txt";
const LINE_COUNT: u16 = 1000;

pub fn day1_p1() {
    let mut file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(&mut file);

    // avoid reallocations with "with_capacity", probably not consequential.
    let mut left_nums = Vec::with_capacity(LINE_COUNT as usize);
    let mut right_nums = Vec::with_capacity(LINE_COUNT as usize);

    // use "map_while" instead of flatten because clippy was annoyed
    for line in br.lines().map_while(Result::ok) {
        // okay i could've used "split_whitespace" but "split_once" is nicer.
        let (left, right) = line
            .split_once(WHITESPACE)
            .expect("Failed to split string!");
        left_nums.push(
            left.parse::<u32>()
                .expect("Failed to parse left number to u32!"),
        );
        right_nums.push(
            right
                .parse::<u32>()
                .expect("Failed to parse right number to u32!"),
        );
    }
    left_nums.sort_unstable(); // use "sort_unstable" instead of sort for primitive types!
    right_nums.sort_unstable();

    let mut sum = 0;
    for idx in 0..left_nums.len() {
        // use absolute difference bcs we aren't just subtracting them
        let difference = left_nums[idx].abs_diff(right_nums[idx]);
        sum += difference;
    }
    println!("Day 1 difference: {sum}");
}

pub fn day1_p2() {
    let mut file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(&mut file);
    let mut left_nums = Vec::with_capacity(LINE_COUNT as usize);
    let mut right_nums = Vec::with_capacity(LINE_COUNT as usize);

    for line in br.lines().map_while(Result::ok) {
        let (left, right) = line
            .split_once(WHITESPACE)
            .expect("Failed to split string!");
        left_nums.push(
            left.parse::<u32>()
                .expect("Failed to parse left number to u32!"),
        );
        right_nums.push(
            right
                .parse::<u32>()
                .expect("Failed to parse right number to u32!"),
        );
    }
    let mut sum = 0;
    for num in left_nums {
        let occurences = right_nums.iter().filter(|r| **r == num);
        let similarity_score = u32::try_from(occurences.count())
            .expect("Failed to convert similarity score to u32!")
            * num;
        sum += similarity_score;
    }
    println!("Day 1 similarity score: {sum}");
}
