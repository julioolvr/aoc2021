use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, BufRead},
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
    let part_1 = board.points_with_overlap().len();
    println!("Part 1: {}", part_1);
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

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        let x_iterator: Box<dyn DoubleEndedIterator<Item = usize>> = if self.from.0 <= self.to.0 {
            Box::new(self.from.0..=self.to.0)
        } else {
            Box::new((self.to.0..=self.from.0).rev())
        };

        for x in x_iterator {
            let y_iterator: Box<dyn DoubleEndedIterator<Item = usize>> = if self.from.1 <= self.to.1
            {
                Box::new(self.from.1..=self.to.1)
            } else {
                Box::new((self.to.1..=self.from.1).rev())
            };

            for y in y_iterator {
                points.push(Point(x, y));
            }
        }

        points
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

    fn points_with_overlap(&self) -> HashSet<Point> {
        let mut points: HashMap<Point, usize> = HashMap::new();

        // For part 1 we only care about lines that are either horizontal or vertical
        for line in self.lines.iter().filter(|line| !line.is_diagonal()) {
            for point in line.points() {
                *points.entry(point).or_insert(0) += 1;
            }
        }

        points
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(point, _)| point)
            .collect()
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
