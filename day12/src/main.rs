use std::{env, str::FromStr};

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

    let mut graph = Graph::new(file.lines().map(|line| line.parse().unwrap()).collect());
    let part_1 = graph.paths_count();
    println!("Part 1: {}", part_1);
}

struct Path {
    from: String,
    to: String,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slices = s.split("-");
        Ok(Path {
            from: slices.next().unwrap().to_string(),
            to: slices.next().unwrap().to_string(),
        })
    }
}

struct Graph {
    paths: Vec<Path>,
    visited: Vec<String>,
}

impl Graph {
    fn new(paths: Vec<Path>) -> Self {
        Graph {
            paths,
            visited: vec![],
        }
    }

    fn paths_count(&mut self) -> usize {
        self.visited = vec![];
        self.paths_count_from("start", 0)
    }

    fn paths_count_from(&mut self, from: &str, i: usize) -> usize {
        if from == "end" {
            return 1;
        }

        if !self.visited.contains(&from.to_owned()) && from.chars().all(char::is_lowercase) {
            self.visited.push(from.to_string());
        }

        let mut targets: Vec<String> = self
            .paths
            .iter()
            .filter(|path| {
                path.to != "start" && path.from == from && !self.visited.contains(&path.to)
            })
            .map(|path| path.to.clone())
            .collect();

        let targets_to_here: Vec<String> = self
            .paths
            .iter()
            .filter(|path| {
                path.from != "start" && path.to == from && !self.visited.contains(&path.from)
            })
            .map(|path| path.from.clone())
            .collect();

        targets.extend(targets_to_here);

        let count: usize = targets
            .iter()
            .map(|target| self.paths_count_from(target, i + 1))
            .sum();

        if self.visited.last() == Some(&from.to_string()) {
            self.visited.pop();
        }

        count
    }
}
