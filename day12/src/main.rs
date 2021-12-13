use std::{collections::HashMap, env, str::FromStr};

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
    let part_1 = graph.paths_count(1);
    println!("Part 1: {}", part_1);

    let part_2 = graph.paths_count(2);
    println!("Part 2: {}", part_2);
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
    visited: HashMap<String, usize>,
}

impl Graph {
    fn new(paths: Vec<Path>) -> Self {
        Graph {
            paths,
            visited: HashMap::new(),
        }
    }

    fn paths_count(&mut self, max_visits_for_single_small_cave: usize) -> usize {
        self.visited = HashMap::new();
        self.paths_count_from("start", max_visits_for_single_small_cave)
    }

    fn paths_count_from(&mut self, from: &str, max_visits_for_single_small_cave: usize) -> usize {
        if from == "end" {
            return 1;
        }

        if from.chars().all(char::is_lowercase) {
            *self.visited.entry(from.to_string()).or_insert(0) += 1;
        }

        let max_allowed_visits = if self
            .visited
            .values()
            .any(|value| *value == max_visits_for_single_small_cave)
        {
            1
        } else {
            max_visits_for_single_small_cave
        };

        let mut targets: Vec<String> = self
            .paths
            .iter()
            .filter(|path| {
                path.to != "start"
                    && path.from == from
                    && self.visited.get(&path.to).unwrap_or(&0) < &max_allowed_visits
            })
            .map(|path| path.to.clone())
            .collect();

        let targets_to_here: Vec<String> = self
            .paths
            .iter()
            .filter(|path| {
                path.from != "start"
                    && path.to == from
                    && self.visited.get(&path.from).unwrap_or(&0) < &max_allowed_visits
            })
            .map(|path| path.from.clone())
            .collect();

        targets.extend(targets_to_here);

        let count: usize = targets
            .iter()
            .map(|target| self.paths_count_from(target, max_visits_for_single_small_cave))
            .sum();

        self.visited
            .entry(from.to_string())
            .and_modify(|count| *count -= 1);

        count
    }
}
