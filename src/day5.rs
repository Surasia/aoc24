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

fn is_valid_rule(rule: &[u32], nodes: &HashMap<u32, Node>) -> bool {
    rule.windows(2).all(|window| {
        if let Some(k) = nodes.get(&window[1]) {
            k.after.contains(&window[0])
        } else {
            false
        }
    })
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

    let sum: u32 = rules
        .iter()
        .filter(|rule| is_valid_rule(rule, &nodes))
        .map(|rule| rule[rule.len() / 2])
        .sum();
    println!("Day 5 unordered sum: {sum}");
}

pub fn day5_p2() {
    let file = File::open(PATH_TO_INPUT).expect("Failed to open file!");
    let br = BufReader::new(file);
    let (pages, rules) = get_pages_and_rules(br);
    let nodes = map_into_nodes(&pages);

    let sum: u32 = rules
        .into_iter()
        .filter(|rule| !is_valid_rule(rule, &nodes))
        .map(|mut rule| {
            rule.sort_by(|s, m| match (nodes.get(s), nodes.get(m)) {
                (Some(k), Some(m)) if k.before.contains(&m.val) => 0.cmp(&1),
                _ => 1.cmp(&0),
            });
            rule[rule.len() / 2]
        })
        .sum();

    println!("Day 5 reordered sum: {sum}");
}
