use std::{collections::HashMap, env};

use anyhow::anyhow;
use cached::{proc_macro::cached, UnboundCache};
use regex::Regex;

fn main() {
    let file = if env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--sample")
    {
        include_str!("../sample.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut lines = file.lines();
    let template = lines.next().unwrap();

    // Skip empty line
    lines.next();

    let insertion_rules = parse_rules(lines);

    let char_counts = counts(template, 10, &insertion_rules);
    let min = char_counts.values().min().unwrap();
    let max = char_counts.values().max().unwrap();

    let part_1 = max - min;
    println!("Part 1: {}", part_1);

    let char_counts = counts(template, 40, &insertion_rules);
    let min = char_counts.values().min().unwrap();
    let max = char_counts.values().max().unwrap();

    let part_2 = max - min;
    println!("Part 2: {}", part_2);
}

fn counts(
    template: &'static str,
    steps: usize,
    rules: &HashMap<(char, char), char>,
) -> HashMap<char, usize> {
    let chars: Vec<char> = template.chars().collect();
    let mut result = HashMap::new();

    for window in chars.as_slice().windows(2) {
        for (char, count) in count_pair((window[0], window[1]), steps, rules) {
            *result.entry(char).or_insert(0) += count;
        }
    }

    // All characters except for the first and the last get counted twice (once in each window
    // they're part of) so we make up for that in this loop
    for char in template[1..template.len() - 1].chars() {
        *result.get_mut(&char).unwrap() -= 1;
    }

    result
}

#[cached(
    type = "UnboundCache<(char, char, usize), HashMap<char, usize>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (first, second, steps) }"#
)]
fn count_pair(
    (first, second): (char, char),
    steps: usize,
    rules: &HashMap<(char, char), char>,
) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    match rules.get(&(first, second)) {
        Some(insert) if steps > 0 => {
            for (char, count) in count_pair((first, *insert), steps - 1, rules) {
                *result.entry(char).or_insert(0) += count;
            }

            for (char, count) in count_pair((*insert, second), steps - 1, rules) {
                *result.entry(char).or_insert(0) += count;
            }

            *result.get_mut(insert).unwrap() -= 1;
        }
        _ => {
            // No more steps, or simply no rule to expand the given pair
            result.insert(first, 1);
            *result.entry(second).or_insert(0) += 1;
        }
    }

    result
}

fn parse_rules(rules: impl Iterator<Item = &'static str>) -> HashMap<(char, char), char> {
    let re = Regex::new(r"(?P<given>\w\w) -> (?P<insert>\w)").unwrap();
    rules
        .map(|line| {
            let captures = re
                .captures(line)
                .ok_or(anyhow!("Unable to match pair insertion rule"))
                .unwrap();

            let mut given_chars = captures["given"].chars();
            let given = (given_chars.next().unwrap(), given_chars.next().unwrap());
            let insert = captures["insert"].chars().next().unwrap();
            (given, insert)
        })
        .collect()
}
