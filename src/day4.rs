#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/* Advent of Code - Day 4 - Surasia */

const PATH_TO_INPUT: &str = "files/day4.txt";
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];
const CHARACTER_COUNT: usize = 140;

pub fn day4_p1() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let mut xmas_count = 0;
    let mut lines = Vec::new();
    for line in br.lines().map_while(Result::ok) {
        let mut l = Vec::with_capacity(CHARACTER_COUNT);
        line.chars().for_each(|c| l.push(c));
        lines.push(l);
        xmas_count += line.matches("XMAS").count(); // lets first find the horizontal matches
        xmas_count += line.matches("SAMX").count(); // and backwards
    }

    for (idx, line) in lines.iter().enumerate() {
        for (index, character) in line.iter().enumerate() {
            // check to make sure that vertical lines + 3 doesn't go above capacity
            // this part checks vertical matches
            if line.len() - 3 > idx {
                let thing = [
                    character.to_owned(),
                    lines[idx + 1][index],
                    lines[idx + 2][index],
                    lines[idx + 3][index],
                ];

                if (thing == XMAS) | (thing == SAMX) {
                    xmas_count += 1;
                }
            }

            // check to make sure that (lines + 3) and (line + 3)  doesn't go above capacity
            // this part checks matches diagonally to the right
            if (line.len() - 3 > index) && ((lines.len() - 3) > idx) {
                let thing = [
                    character.to_owned(),
                    lines[idx + 1][index + 1],
                    lines[idx + 2][index + 2],
                    lines[idx + 3][index + 3],
                ];
                if (thing == XMAS) | (thing == SAMX) {
                    xmas_count += 1;
                }
            }

            // this part checks matches diagonally to the right
            if (2 < index) && ((lines.len() - 3) > idx) {
                let thing = [
                    character.to_owned(),
                    lines[idx + 1][index - 1],
                    lines[idx + 2][index - 2],
                    lines[idx + 3][index - 3],
                ];
                if (thing == XMAS) | (thing == SAMX) {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("Day 4 XMAS count: {xmas_count}");
}

pub fn day4_p2() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let mut xmas_count = 0;
    let mut vertical_lines = Vec::new();
    for line in br.lines().map_while(Result::ok) {
        let mut l = Vec::new();
        line.chars().for_each(|c| l.push(c));
        vertical_lines.push(l);
    }

    for (idx, vert) in vertical_lines.iter().enumerate() {
        for (index, horiz) in vert.iter().enumerate() {
            let mut flipped = false;

            // first just check if our character in the center is 'A'
            if (horiz == &'A')
                && idx < vert.len() - 1 // make sure that MAS/SAM doesn't go out of bounds
                && idx > 0
                && index < vert.len() - 1
                && index > 0
            {
                let left_up = vertical_lines[idx + 1][index - 1];
                match left_up {
                    'M' => {}
                    'S' => {
                        flipped = true; // each SAM/MAS can be flipped. this is for the first one
                    }
                    _ => continue,
                }
                let right_down = vertical_lines[idx - 1][index + 1];
                match right_down {
                    'M' => {
                        if !flipped {
                            continue; // M-A-M not valid
                        }
                    }
                    'S' => {
                        if flipped {
                            continue; // S-A-S not valid
                        }
                    }
                    _ => continue,
                }

                flipped = false; // now we'll check our next SAM/MAS
                let right_up = vertical_lines[idx + 1][index + 1];
                match right_up {
                    'M' => flipped = true,
                    'S' => {}
                    _ => continue,
                }
                let left_down = vertical_lines[idx - 1][index - 1];
                match left_down {
                    'M' => {
                        if flipped {
                            continue; // M-A-M not valid
                        }
                    }
                    'S' => {
                        if !flipped {
                            continue; // S-A-S not valid
                        }
                    }
                    _ => continue,
                }
                xmas_count += 1;
            }
        }
    }

    println!("Day 4 X-MAS count: {xmas_count}");
}
