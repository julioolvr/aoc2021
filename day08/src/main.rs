use anyhow::{anyhow, bail};
use std::{collections::HashSet, convert::TryFrom, env, str::FromStr};

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

    let signals: Vec<Signal> = file
        .lines()
        .filter(|line| line.trim() != "")
        .map(|line| line.parse().unwrap())
        .collect();

    let part_1: usize = signals
        .iter()
        .map(|signal| signal.trivial_digits_in_output())
        .sum();

    println!("Part 1: {}", part_1);
}

#[derive(Debug)]
struct Signal {
    input: Vec<Digit>,
    output: Vec<Digit>,
}

impl Signal {
    fn trivial_digits_in_output(&self) -> usize {
        self.output
            .iter()
            .filter(|digit| digit.is_trivial())
            .count()
    }
}

impl FromStr for Signal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('|');
        let input = split
            .next()
            .ok_or(anyhow!("Couldn't extract input digits from signal string"))?;
        let output = split
            .next()
            .ok_or(anyhow!("Couldn't extract output digits from signal string"))?;

        let input = input
            .split(' ')
            .map(|digit| digit.trim())
            .filter(|digit| digit != &"")
            .map(|digit| digit.parse())
            .collect::<anyhow::Result<Vec<Digit>>>()?;
        let output = output
            .split(' ')
            .map(|digit| digit.trim())
            .filter(|digit| digit != &"")
            .map(|digit| digit.parse())
            .collect::<anyhow::Result<Vec<Digit>>>()?;

        Ok(Signal { input, output })
    }
}

#[derive(Debug)]
struct Digit {
    enabled_segments: HashSet<Segment>,
}

impl Digit {
    fn is_trivial(&self) -> bool {
        match self.enabled_segments.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }
}

impl FromStr for Digit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digit {
            enabled_segments: s
                .chars()
                .map(|c| Segment::try_from(c))
                .collect::<anyhow::Result<HashSet<Segment>>>()?,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let segment = match value {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => bail!("Invalid char for segment: {}", value),
        };

        Ok(segment)
    }
}
