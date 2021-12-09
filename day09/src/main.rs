use std::env;

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

    let map: Vec<Vec<usize>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let height = map.len();
    let width = map.first().unwrap().len();

    let minimums: Vec<usize> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, n)| {
                    if x > 0 && map[y][x - 1] <= *n {
                        None
                    } else if y > 0 && map[y - 1][x] <= *n {
                        None
                    } else if x < (width - 1) && map[y][x + 1] <= *n {
                        None
                    } else if y < (height - 1) && map[y + 1][x] <= *n {
                        None
                    } else {
                        Some(n)
                    }
                })
                .copied()
                .collect::<Vec<usize>>()
        })
        .collect();

    let part_1: usize = minimums.iter().map(|n| n + 1).sum();
    println!("Part 1: {}", part_1);
}
