use std::io::BufRead;
use std::{env, fs::File, io};

fn main() {
    let lines: Vec<String> = read_lines()
        .expect("Error reading file")
        .map(|line| line.expect("Error reading line"))
        .collect();

    let length = lines.first().unwrap().len();
    let mut result = vec![0; length];

    for line in lines {
        for (i, bit) in line.chars().enumerate() {
            match bit {
                '0' => result[i] -= 1,
                '1' => result[i] += 1,
                _ => panic!("Unexpected bit {}", bit),
            }
        }
    }

    let gamma_rate = result
        .iter()
        .map(|bit_count| {
            // The problem description doesn't mention what happens if there's exactly the same
            // number of 1 and 0 in one of the columns (both the sample and the input files have an
            // even amount of lines, so it could happen). So this code assumes that won't ever be
            // the case (if so, bit_count would be 0).
            if *bit_count > 0 {
                1
            } else {
                0
            }
        })
        .fold(0, |acc, bit| acc << 1 | bit);
    let epsilon_rate = !gamma_rate & ((1 << length) - 1);

    println!("Day 1: {}", gamma_rate * epsilon_rate);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
