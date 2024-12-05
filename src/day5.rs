#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/* Advent of Code - Day 5 - Surasia */

const PATH_TO_INPUT: &str = "files/day5.txt";

#[derive(Debug)]
struct Node {
    val: u32,
    before: Vec<u32>,
    after: Vec<u32>,
}

fn get_pages_and_rules<R: BufRead>(input: R) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut pages = Vec::new();
    let mut rules = Vec::new();
    for line in input.lines().map_while(Result::ok) {
        pages.push(
            line.split('|')
                .filter(|s| !s.is_empty() & !s.contains(',')) // page order
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
        rules.push(
            line.split(',')
                .filter(|s| !s.is_empty() & !s.contains('|')) // rules
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }
    pages.retain(|p| !p.is_empty()); // make sure we remove empty arrays
    rules.retain(|r| !r.is_empty());
    (pages, rules)
}

fn map_into_nodes(pages: &Vec<Vec<u32>>) -> HashMap<u32, Node> {
    let mut nodes = HashMap::new();
    for page in pages {
        let after = pages
            .iter()
            .filter(|s| s[1] == page[0])
            .map(|s| s[0])
            .collect::<Vec<_>>();
        let before = pages
            .iter()
            .filter(|s| s[0] == page[0])
            .map(|s| s[1])
            .collect::<Vec<_>>();
        let n1 = Node {
            val: page[0],
            before,
            after,
        };
        let after = pages
            .iter()
            .filter(|s| s[1] == page[1])
            .map(|s| s[0])
            .collect::<Vec<_>>();
        let before = pages
            .iter()
            .filter(|s| s[0] == page[1])
            .map(|s| s[1])
            .collect::<Vec<_>>();
        let n2 = Node {
            val: page[1],
            before,
            after,
        };
        nodes.insert(page[0], n1);
        nodes.insert(page[1], n2);
    }
    nodes
}

pub fn day5_p1() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let (pages, rules) = get_pages_and_rules(br);
    let nodes = map_into_nodes(&pages);

    let mut sum = 0;
    for rule in rules {
        let mut valid = true;
        for (idx, i) in rule.iter().enumerate() {
            let k = nodes.get(i);
            if let Some(k) = k {
                if idx != 0 && !k.after.contains(&rule[idx - 1]) {
                    valid = false;
                }
            }
        }
        if valid {
            let midpoint = rule[rule.len() / 2]; // kind of a hacky way to get the middle
            sum += midpoint;
        }
    }
    println!("Day 5 unordered sum: {sum}");
}

pub fn day5_p2() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let (pages, rules) = get_pages_and_rules(br);
    let nodes = map_into_nodes(&pages);
    let mut sum = 0;

    for mut rule in rules {
        let mut valid = true;
        for (idx, i) in rule.iter().enumerate() {
            let k = nodes.get(i);
            if let Some(k) = k {
                if idx != 0 && !k.after.contains(&rule[idx - 1]) {
                    valid = false;
                }
            }
        }
        if !valid {
            rule.sort_by(|s, m| {
                let k = nodes.get(s);
                let m = nodes.get(m);
                if let Some(k) = k {
                    if let Some(m) = m {
                        if k.before.contains(&m.val) {
                            return 0.cmp(&1);
                        }
                    }
                }

                1.cmp(&0) // sort_by requires something that implements "cmp", so...
            });
            let midpoint = rule[rule.len() / 2];
            sum += midpoint;
        }
    }
    println!("Day 5 reordered sum: {sum}");
}
