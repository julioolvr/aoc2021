use anyhow::{anyhow, bail};
use regex::Regex;
use std::{collections::HashSet, env, fmt, str::FromStr};

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
    let points: HashSet<Coordinates> = lines
        .by_ref()
        .take_while(|line| line != &"")
        .map(|line| line.parse().unwrap())
        .collect();
    let mut paper = Paper::new(points);

    let mut instructions = lines.map(|line| line.parse().unwrap());
    paper = paper.fold(instructions.next().unwrap());
    let part_1 = paper.points.len();
    println!("Part 1: {}", part_1);
}

#[derive(PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl FromStr for Coordinates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let x: usize = parts
            .next()
            .ok_or(anyhow!("Could not find x coordinate"))?
            .parse()?;
        let y: usize = parts
            .next()
            .ok_or(anyhow!("Could not find y coordinate"))?
            .parse()?;
        Ok(Coordinates { x, y })
    }
}

impl From<(usize, usize)> for Coordinates {
    fn from((x, y): (usize, usize)) -> Self {
        Coordinates { x, y }
    }
}

struct Paper {
    points: HashSet<Coordinates>,
    width: usize,
    height: usize,
}

impl Paper {
    fn new(points: HashSet<Coordinates>) -> Self {
        let width = points.iter().map(|point| point.x).max().unwrap() + 1;
        let height = points.iter().map(|point| point.y).max().unwrap() + 1;
        Paper {
            points,
            width,
            height,
        }
    }

    fn fold(self, instruction: Instruction) -> Paper {
        let height = self.height;
        let width = self.width;

        let partitioner: Box<dyn Fn(&Coordinates) -> bool> = match instruction {
            Instruction::X(index) => Box::new(move |point| point.x < index),
            Instruction::Y(index) => Box::new(move |point| point.y < index),
        };

        let (mut new_points, second_half): (HashSet<Coordinates>, HashSet<Coordinates>) =
            self.points.into_iter().partition(partitioner);

        new_points.extend(second_half.into_iter().map(|point| match instruction {
            Instruction::X(_) => (width - point.x - 1, point.y).into(),
            Instruction::Y(_) => (point.x, height - point.y - 1).into(),
        }));

        Paper::new(new_points)
    }
}

enum Instruction {
    X(usize),
    Y(usize),
}

/// Parsing and debugging

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (axis, value) = match self {
            &Instruction::X(value) => ("X", value),
            &Instruction::Y(value) => ("Y", value),
        };

        write!(f, "{}={}", axis, value)
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<axis>[xy])=(?P<value>\d+)")?;
        let captures = re
            .captures(s)
            .ok_or(anyhow!("Invalid instruction string `{}`", s))?;
        let axis = &captures["axis"];
        let value: usize = captures["value"].parse()?;

        match axis {
            "x" => Ok(Instruction::X(value)),
            "y" => Ok(Instruction::Y(value)),
            _ => bail!("Invalid axis {}", axis),
        }
    }
}

impl fmt::Debug for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.points.contains(&(x, y).into()) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
