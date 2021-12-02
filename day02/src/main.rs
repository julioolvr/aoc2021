use anyhow::{anyhow, bail};
use std::io::BufRead;
use std::str::FromStr;
use std::{env, fs::File, io};

fn main() {
    let commands: Vec<Command> = read_lines()
        .expect("Error reading file")
        .map(|line| {
            line.expect("Error reading line")
                .parse::<Command>()
                .expect("Error parsing command")
        })
        .collect();

    let mut submarine = Submarine::new();
    submarine.navigate(commands.iter());
    println!("Part 1: {}", submarine.result());

    let mut submarine = Submarine::new();
    submarine.navigate_with_aim(commands.iter());
    println!("Part 2: {}", submarine.result());
}

struct Submarine {
    position: Position,
    aim: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            position: Position::new(),
            aim: 0,
        }
    }

    fn result(&self) -> i64 {
        self.position.depth * self.position.horizontal as i64
    }

    fn navigate<'a>(&mut self, commands: impl Iterator<Item = &'a Command>) {
        for command in commands {
            self.position.apply_command(&command);
        }
    }

    fn navigate_with_aim<'a>(&mut self, commands: impl Iterator<Item = &'a Command>) {
        for command in commands {
            self.aim = self.position.apply_command_with_aim(&command, self.aim);
        }
    }
}

struct Position {
    horizontal: u64,
    depth: i64,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Forward(n) => self.horizontal += n,
            Command::Down(n) => self.depth += *n as i64,
            Command::Up(n) => self.depth -= *n as i64,
        }
    }

    fn apply_command_with_aim(&mut self, command: &Command, aim: i64) -> i64 {
        match command {
            Command::Forward(n) => {
                self.horizontal += n;
                self.depth += aim * *n as i64;
                aim
            }
            Command::Down(n) => aim + *n as i64,
            Command::Up(n) => aim - *n as i64,
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
