use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines: Vec<u64> = read_lines()
        .expect("Error reading file")
        .map(|line| {
            line.expect("Error reading line")
                .parse()
                .expect("Error parsing line as number")
        })
        .collect();

    let part_1 = part_1(lines.iter().cloned());
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&lines);
    println!("Part 2: {}", part_2);
}

fn part_1(measurements: impl Iterator<Item = u64>) -> u64 {
    count_increases(measurements)
}

fn part_2(measurements: &[u64]) -> u64 {
    count_increases(measurements.windows(3).map(|window| window.iter().sum()))
}

fn count_increases(mut measurements: impl Iterator<Item = u64>) -> u64 {
    let first_measurement = measurements
        .next()
        .expect("Unexpected empty measurements iterator");

    measurements
        .fold(
            (0, first_measurement),
            |(increases, previous_measurement), current_measurement| {
                let next_increases = if current_measurement > previous_measurement {
                    increases + 1
                } else {
                    increases
                };

                (next_increases, current_measurement)
            },
        )
        .0
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
