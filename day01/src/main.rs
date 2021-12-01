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

    let part_2 = part_2(lines.iter().cloned());
    println!("Part 2: {}", part_2);
}

fn part_1(mut measurements: impl Iterator<Item = u64>) -> u64 {
    let first_depth = measurements
        .next()
        .expect("Unexpected empty measurements iterator");

    let increases = measurements.fold(
        (0, first_depth),
        |(increases, previous_depth), current_depth| {
            let next_increases = if current_depth > previous_depth {
                increases + 1
            } else {
                increases
            };

            (next_increases, current_depth)
        },
    );

    increases.0
}

fn part_2(mut measurements: impl Iterator<Item = u64>) -> u64 {
    let first_window = (
        measurements
            .next()
            .expect("Unexpected empty measurements iterator"),
        measurements
            .next()
            .expect("Unexpected empty measurements iterator"),
        measurements
            .next()
            .expect("Unexpected empty measurements iterator"),
    );

    let increases = measurements.fold(
        (0, first_window),
        |(increases, previous_window), current_depth| {
            let previous_window_sum = previous_window.0 + previous_window.1 + previous_window.2;
            let current_window_sum = previous_window.1 + previous_window.2 + current_depth;

            let next_increases = if current_window_sum > previous_window_sum {
                increases + 1
            } else {
                increases
            };

            (
                next_increases,
                (previous_window.1, previous_window.2, current_depth),
            )
        },
    );

    increases.0
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
