use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
    iter,
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let lines: Vec<Line> = read_lines()
        .expect("Error reading file")
        .map(|line| {
            line.expect("Error reading line")
                .parse()
                .expect("Unable to parse as line")
        })
        .collect();

    let board = Board::new(lines);
    println!("Part 1: {}", board.part_1());
    println!("Part 2: {}", board.part_2());
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.from.1 == self.to.1
    }

    fn is_vertical(&self) -> bool {
        self.from.0 == self.to.0
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let x_iterator: Box<dyn Iterator<Item = usize>> = if self.from.0 < self.to.0 {
            Box::new(self.from.0..=self.to.0)
        } else if self.from.0 > self.to.0 {
            Box::new((self.to.0..=self.from.0).rev())
        } else {
            Box::new(iter::repeat(self.from.0))
        };

        let y_iterator: Box<dyn Iterator<Item = usize>> = if self.from.1 < self.to.1 {
            Box::new(self.from.1..=self.to.1)
        } else if self.from.1 > self.to.1 {
            Box::new((self.to.1..=self.from.1).rev())
        } else {
            Box::new(iter::repeat(self.from.1))
        };

        x_iterator.zip(y_iterator).map(|(x, y)| Point(x, y))
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let from: Point = chars
            .by_ref()
            .take_while(|c| *c != ' ')
            .collect::<String>()
            .parse()?;
        chars.by_ref().skip_while(|char| *char != ' ').next();
        let to: Point = chars.collect::<String>().parse()?;

        Ok(Line { from, to })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x: usize = split.next().unwrap().parse()?;
        let y: usize = split.next().unwrap().parse()?;
        Ok(Point(x, y))
    }
}

struct Board {
    lines: Vec<Line>,
}

impl Board {
    fn new(lines: Vec<Line>) -> Self {
        Board { lines }
    }

    fn part_1(&self) -> usize {
        self.lines
            .iter()
            // For part 1 we only care about lines that are either horizontal or vertical
            .filter(|line| !line.is_diagonal())
            .flat_map(|line| line.points())
            .fold(HashMap::<Point, usize>::new(), |mut points, point| {
                *points.entry(point).or_insert(0) += 1;
                points
            })
            .values()
            .filter(|count| **count >= 2)
            .count()
    }

    fn part_2(&self) -> usize {
        self.lines
            .iter()
            // For part 2 we use all lines
            .flat_map(|line| line.points())
            .fold(HashMap::<Point, usize>::new(), |mut points, point| {
                *points.entry(point).or_insert(0) += 1;
                points
            })
            .values()
            .filter(|count| **count >= 2)
            .count()
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
