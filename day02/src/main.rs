use anyhow::{anyhow, bail};
use std::io::BufRead;
use std::str::FromStr;
use std::{env, fs::File, io};

fn main() {
    let commands = read_lines().expect("Error reading file").map(|line| {
        line.expect("Error reading line")
            .parse::<Command>()
            .expect("Error parsing command")
    });

    let position = commands.fold(Position::new(), |mut position, command| {
        position.apply_command(&command);
        position
    });

    println!("Part 1: {}", position.result());
}

struct Position {
    horizontal: u64,
    depth: u64,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    fn result(&self) -> u64 {
        self.horizontal * self.depth
    }

    fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Forward(n) => self.horizontal += n,
            Command::Down(n) => self.depth += n,
            Command::Up(n) => self.depth -= n,
        }
    }
}

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let (direction, value) = match &parts[..] {
            &[direction, value, ..] => (direction, value),
            _ => bail!("Parse error"),
        };
        let value = value.parse::<u64>()?;

        match direction {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(anyhow!("Invalid direction: {}", direction)),
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
