use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let mut lines = read_lines().expect("Error reading file").map(|line| {
        line.expect("Error reading line")
            .parse::<u64>()
            .expect("Error parsing line as number")
    });

    let first_depth = lines.next().expect("Unexpected empty lines iterator");

    let increases = lines.fold(
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

    println!("Part 1: {}", increases.0);

    Ok(())
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
