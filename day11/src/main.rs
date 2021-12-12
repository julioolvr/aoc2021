use std::{env, fmt::Debug, str::FromStr};

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

    let map: Map = file.parse().unwrap();
    let mut simulator = simulate(map);

    let part_1: usize = simulator.by_ref().take(100).sum();
    println!("Part 1: {}", part_1);

    let part_2 = simulator
        .enumerate()
        .find_map(|(i, flashes)| {
            if flashes == 100 {
                // We add 1 because i is zero sized, but the first step per the problem definition
                // is 1. We add 100 because we consumed the first 100 for part 1 and we only
                // start counting when iterating for part 2.
                Some(i + 1 + 100)
            } else {
                None
            }
        })
        .unwrap();
    println!("Part 2: {}", part_2);
}

type Coordinates = (usize, usize);

struct Map {
    octopuses: Vec<Octopus>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(octopuses: Vec<Octopus>) -> Self {
        // Per the problem statement, the map is 10 by 10
        Map {
            octopuses: octopuses,
            width: 10,
            height: 10,
        }
    }

    fn to_coordinates(&self, index: usize) -> Coordinates {
        (index % self.width, index / self.width)
    }

    fn to_index(&self, (x, y): Coordinates) -> usize {
        y * self.width + x
    }

    fn at(&mut self, coordinates: Coordinates) -> Option<&mut Octopus> {
        let index = self.to_index(coordinates);
        self.octopuses.get_mut(index)
    }

    fn step(&mut self) -> usize {
        let mut flashes = 0;

        for octopus in &mut self.octopuses {
            octopus.energy += 1;
        }

        let mut should_check_for_new_flashes = true;

        while should_check_for_new_flashes {
            should_check_for_new_flashes = false;

            for i in 0..self.octopuses.len() {
                let (x, y) = self.to_coordinates(i);
                let octopus = &mut self.octopuses[i];

                if !octopus.flashed && octopus.energy >= 10 {
                    flashes += 1;
                    should_check_for_new_flashes = true;
                    octopus.flashed = true;

                    if x > 0 {
                        self.at((x - 1, y)).unwrap().energy += 1;
                    }

                    if x > 0 && y > 0 {
                        self.at((x - 1, y - 1)).unwrap().energy += 1;
                    }

                    if y > 0 {
                        self.at((x, y - 1)).unwrap().energy += 1;
                    }

                    if x < self.width - 1 && y > 0 {
                        self.at((x + 1, y - 1)).unwrap().energy += 1;
                    }

                    if x < self.width - 1 {
                        self.at((x + 1, y)).unwrap().energy += 1;
                    }

                    if x < self.width - 1 && y < self.height - 1 {
                        self.at((x + 1, y + 1)).unwrap().energy += 1;
                    }

                    if y < self.height - 1 {
                        self.at((x, y + 1)).unwrap().energy += 1;
                    }

                    if x > 0 && y < self.height - 1 {
                        self.at((x - 1, y + 1)).unwrap().energy += 1;
                    }
                }
            }
        }

        for octopus in &mut self.octopuses {
            octopus.flashed = false;
            if octopus.energy >= 10 {
                octopus.energy = 0;
            }
        }

        flashes
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octopuses = s.lines().flat_map(|line| {
            line.chars()
                // Assumes all chars will be digits, per the problem statement
                .map(|c| c.to_digit(10).unwrap() as usize)
                .map(|energy| Octopus::new(energy))
        });

        Ok(Map::new(octopuses.collect()))
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let octopus = &self.octopuses[y * self.width + x];
                if octopus.energy == 10 {
                    write!(f, "X")?;
                } else if octopus.energy > 10 {
                    write!(f, "x")?;
                } else {
                    write!(f, "{}", octopus.energy)?;
                }
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

struct Octopus {
    energy: usize,
    flashed: bool,
}

impl Octopus {
    fn new(energy: usize) -> Octopus {
        Octopus {
            energy,
            flashed: false,
        }
    }
}

struct StepIterator {
    map: Map,
}

fn simulate(map: Map) -> StepIterator {
    StepIterator { map }
}

impl Iterator for StepIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(self.map.step())
    }
}
