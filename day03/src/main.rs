use std::io::BufRead;
use std::iter::FromIterator;
use std::ops;
use std::{env, fs::File, io};

fn main() {
    let lines: Vec<Diagnostic> = read_lines()
        .expect("Error reading file")
        .map(|line| {
            line.expect("Error reading line")
                .chars()
                .map(|bit| match bit {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Unexpected bit {}", bit),
                })
                .collect()
        })
        .collect();

    let submarine = Submarine::new(lines.clone());

    let power_consumption = submarine.power_consumption().total();
    println!("Part 1: {}", power_consumption);

    let life_support_rating = submarine.life_support_rating().total();
    println!("Part 2: {}", life_support_rating);
}

struct Submarine {
    diagnostics: Vec<Diagnostic>,
    diagnostic_length: usize,
    bit_counts: BitCounts,
}

impl Submarine {
    fn new(diagnostics: Vec<Diagnostic>) -> Self {
        let diagnostic_length = diagnostics
            .first()
            .expect("Unexpected empty diagnostics")
            .len();

        let bit_counts = BitCounts::from_diagnostics(&diagnostics);

        Submarine {
            diagnostics,
            diagnostic_length,
            bit_counts,
        }
    }

    fn power_consumption(&self) -> PowerConsumption {
        let gamma_rate = (0..self.diagnostic_length)
            .map(|bit_position| self.bit_counts.most_common_at(bit_position))
            .fold(0, |acc, bit| acc << 1 | bit);

        let epsilon_rate = !gamma_rate & ((1 << self.diagnostic_length) - 1);

        PowerConsumption {
            gamma_rate,
            epsilon_rate,
        }
    }

    fn life_support_rating(&self) -> LifeSupportRating {
        LifeSupportRating {
            co2_scrubber_rating: self.co2_scrubber_rating(),
            oxygen_generator_rating: self.oxygen_generator_rating(),
        }
    }

    fn oxygen_generator_rating(&self) -> usize {
        self.find_diagnostic(&|bit_counts: BitCounts, index: usize| {
            bit_counts.most_common_at(index) == 1
        })
        .data
        .iter()
        .fold(0, |acc, bit| acc << 1 | (*bit as usize))
    }

    fn co2_scrubber_rating(&self) -> usize {
        self.find_diagnostic(&|bit_counts: BitCounts, index: usize| {
            bit_counts.least_common_at(index) == 1
        })
        .data
        .iter()
        .fold(0, |acc, bit| acc << 1 | (*bit as usize))
    }

    fn find_diagnostic(&self, should_match_1: &dyn Fn(BitCounts, usize) -> bool) -> Diagnostic {
        let mut diagnostic_test = self.diagnostics.clone();
        let mut difference = BitCounts::with_length(self.diagnostic_length);
        let mut i = 0;

        while diagnostic_test.len() > 1 {
            let mut new_difference = difference.clone();

            diagnostic_test = diagnostic_test
                .into_iter()
                .filter(|line| {
                    let bit = line[i];
                    let bit_matches = if should_match_1(&self.bit_counts - &difference, i) {
                        bit
                    } else {
                        !bit
                    };

                    if !bit_matches {
                        new_difference += line;
                        return false;
                    }

                    true
                })
                .collect::<Vec<Diagnostic>>();

            i += 1;
            difference = new_difference;
        }

        diagnostic_test
            .first()
            .expect("Did not find matching diagnostic")
            .clone()
    }
}

#[derive(Clone)]
struct Diagnostic {
    data: Vec<bool>,
}

impl Diagnostic {
    fn new(data: Vec<bool>) -> Self {
        Diagnostic { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl ops::Index<usize> for Diagnostic {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl FromIterator<bool> for Diagnostic {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        Diagnostic::new(iter.into_iter().collect())
    }
}

struct PowerConsumption {
    gamma_rate: usize,
    epsilon_rate: usize,
}

impl PowerConsumption {
    fn total(&self) -> usize {
        self.gamma_rate * self.epsilon_rate
    }
}

struct LifeSupportRating {
    oxygen_generator_rating: usize,
    co2_scrubber_rating: usize,
}

impl LifeSupportRating {
    fn total(&self) -> usize {
        self.oxygen_generator_rating * self.co2_scrubber_rating
    }
}

#[derive(Clone, Debug)]
struct BitCounts {
    bit_count_per_position: Vec<isize>,
}

impl BitCounts {
    fn with_length(length: usize) -> BitCounts {
        BitCounts {
            bit_count_per_position: vec![0; length],
        }
    }

    fn from_diagnostics(diagnostics: &Vec<Diagnostic>) -> BitCounts {
        let diagnostic_length = diagnostics
            .first()
            .expect("Unexpected empty diagnostics")
            .len();

        // For each bit in the input numbers it will have one element - if that element is > 0 that
        // means there are more 1s than 0s in that position, and vice versa if < 0.
        let mut bit_count_per_position = vec![0; diagnostic_length];

        for diagnostic in diagnostics {
            for (i, bit) in diagnostic.data.iter().enumerate() {
                if *bit {
                    bit_count_per_position[i] += 1;
                } else {
                    bit_count_per_position[i] -= 1;
                }
            }
        }

        BitCounts {
            bit_count_per_position,
        }
    }

    fn most_common_at(&self, index: usize) -> usize {
        if self.bit_count_per_position[index] >= 0 {
            1
        } else {
            0
        }
    }

    fn least_common_at(&self, index: usize) -> usize {
        if self.bit_count_per_position[index] >= 0 {
            0
        } else {
            1
        }
    }
}

impl ops::Sub<&BitCounts> for &BitCounts {
    type Output = BitCounts;

    fn sub(self, rhs: &BitCounts) -> BitCounts {
        let length = self.bit_count_per_position.len();
        let mut result = BitCounts::with_length(length);

        if length != rhs.bit_count_per_position.len() {
            panic!("Tried to sub BitCounts of different lengths");
        }

        for i in 0..length {
            result.bit_count_per_position[i] =
                self.bit_count_per_position[i] - rhs.bit_count_per_position[i];
        }

        result
    }
}

impl ops::AddAssign<&Diagnostic> for BitCounts {
    fn add_assign(&mut self, rhs: &Diagnostic) {
        for (i, bit) in rhs.data.iter().enumerate() {
            if *bit {
                self.bit_count_per_position[i] += 1
            } else {
                self.bit_count_per_position[i] -= 1
            }
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
