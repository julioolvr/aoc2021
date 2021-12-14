use std::{collections::HashMap, env, fmt, str::FromStr};

use anyhow::anyhow;
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
    let mut template: Template = lines.next().unwrap().into();

    // Skip empty line
    lines.next();

    let insertion_rules: PairInsertionRules =
        lines.collect::<Vec<&str>>().join("\n").parse().unwrap();

    for _ in 0..10 {
        template.step(&insertion_rules);
    }

    let (min, max) = template.least_and_most_common_element_counts();
    let part_1 = max - min;
    println!("Part 1: {}", part_1);
}

struct Template {
    template: String,
}

impl Template {
    fn step(&mut self, rules: &PairInsertionRules) {
        let mut inserted = 0;
        let mut new_template = self.template.clone();
        let current_chars: Vec<char> = self.template.chars().collect();

        for (i, window) in current_chars.as_slice().windows(2).enumerate() {
            let first = window[0];
            let second = window[1];

            if let Some(insert) = rules.rules.get(&(first, second)) {
                new_template.insert(i + inserted + 1, insert.clone());
                inserted += 1;
            }
        }

        self.template = new_template;
    }

    fn least_and_most_common_element_counts(&self) -> (usize, usize) {
        let mut result = HashMap::new();

        for char in self.template.chars() {
            *result.entry(char).or_insert(0) += 1;
        }

        let min = result.values().min().unwrap();
        let max = result.values().max().unwrap();
        (*min, *max)
    }
}

impl Into<Template> for &str {
    fn into(self) -> Template {
        Template {
            template: self.to_string(),
        }
    }
}

struct PairInsertionRules {
    rules: HashMap<(char, char), char>,
}

impl FromStr for PairInsertionRules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<given>\w\w) -> (?P<insert>\w)")?;
        let map: HashMap<(char, char), char> = s
            .lines()
            .map(|line| {
                let captures = re
                    .captures(line)
                    .ok_or(anyhow!("Unable to match pair insertion rule"))?;

                let mut given_chars = captures["given"].chars();
                let given = (given_chars.next().unwrap(), given_chars.next().unwrap());
                let insert = captures["insert"].chars().next().unwrap();
                Ok((given, insert))
            })
            .collect::<Result<_, anyhow::Error>>()?;

        Ok(PairInsertionRules { rules: map })
    }
}

impl fmt::Debug for PairInsertionRules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (given, insert) in &self.rules {
            writeln!(f, "{}{} -> {}", given.0, given.1, insert)?;
        }

        Ok(())
    }
}
