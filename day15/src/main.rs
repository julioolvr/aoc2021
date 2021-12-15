use std::{
    collections::{HashMap, HashSet},
    env,
    iter::{repeat, FromIterator},
};

use anyhow::anyhow;
// use cached::{proc_macro::cached, UnboundCache};

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

    let map = parse_map(file).unwrap();
    let part_1 = shortest_path_cost(&map);
    println!("Part 1: {}", part_1);

    let expanded_map = expand_map(map);
    let part_2 = shortest_path_cost(&expanded_map);
    println!("Part 2: {}", part_2);
}

fn shortest_path_cost(map: &Vec<Vec<usize>>) -> usize {
    let width = map.first().unwrap().len();
    let height = map.len();

    let mut current = (0, 0);
    let mut unvisited_nodes: HashSet<(usize, usize)> =
        HashSet::from_iter((0..width).flat_map(|x| repeat(x).zip(0..height)));
    let mut costs: HashMap<(usize, usize), usize> =
        HashMap::from_iter([(current, 0)].iter().cloned());
    let target = (width - 1, height - 1);

    while unvisited_nodes.contains(&target) {
        let mut neighbors = vec![];
        let cost = *costs.get(&current).unwrap();
        let (x, y) = current;

        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if x < width - 1 {
            neighbors.push((x + 1, y));
        }

        if y < height - 1 {
            neighbors.push((x, y + 1));
        }

        for (neighbor_x, neighbor_y) in neighbors
            .iter()
            .filter(|neighbor| unvisited_nodes.contains(neighbor))
        {
            let new_cost = cost + map[*neighbor_y][*neighbor_x];

            if *costs.entry((*neighbor_x, *neighbor_y)).or_insert(new_cost) > new_cost {
                costs.insert((*neighbor_x, *neighbor_y), new_cost);
            }
        }

        unvisited_nodes.remove(&current);

        if let Some(new_current) = unvisited_nodes
            .iter()
            .min_by_key(|node| costs.get(node).unwrap_or(&usize::MAX))
        {
            current = new_current.clone();
        } else {
            // All visited
            break;
        }
    }

    *costs.get(&target).unwrap()
}

fn expand_map(map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let expanded_rows: Vec<Vec<usize>> = map
        .into_iter()
        .map(|row| {
            repeat(row)
                .take(5)
                .enumerate()
                .flat_map(|(repeat, row)| row.into_iter().map(move |n| wrap(n + repeat)))
                .collect::<Vec<usize>>()
        })
        .collect();

    repeat(expanded_rows)
        .take(5)
        .enumerate()
        .flat_map(|(repeat, rows_group)| {
            rows_group.into_iter().map(move |row| {
                row.into_iter()
                    .map(|n| wrap(n + repeat))
                    .collect::<Vec<usize>>()
            })
        })
        .collect()
}

fn wrap(n: usize) -> usize {
    (n - 1) % 9 + 1
}

// Parsing and debugging

fn print_map(map: &Vec<Vec<usize>>) {
    for row in map {
        for digit in row {
            print!("{}", digit);
        }
        println!("");
    }
}

fn parse_map(s: &str) -> anyhow::Result<Vec<Vec<usize>>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    char.to_digit(10)
                        .map(|digit| digit as usize)
                        .ok_or(anyhow!("Invalid digit {}", char))
                })
                .collect()
        })
        .collect::<Result<_, _>>()
}
