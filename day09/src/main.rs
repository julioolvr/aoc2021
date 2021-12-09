use std::{collections::HashSet, env, iter::FromIterator};

/**
 * --- Day 9: Smoke Basin ---
 *
 * We receive as input a map of depths (each depth is a single decimal digit). The first part asks
 * to find all the low points in the map - points that have a depth that is lower than its vertical
 * and horizontal neighbors. This is done by the `low_points` function by iterating over each point
 * and comparing it to its neighbors.
 *
 * `low_points` returns both the depth and the coordinates of the low points, so the coordinates can
 * be used in part 2. Part 2 asks for the sizes of the "basins" of each low point. A basin is made
 * of all neighbors of increasing depth, until reaching some point of depth 9 (which is not included
 * in the basin). To calculate this we create a `HashSet` of the points in each basin. We start that
 * set with a low point, and check each neighbor. If they are higher (will be the case in the
 * initial iteration since by definition a low point is surrounded by higher points) we calculate
 * the basin starting from *that* point, and so on until we reach a 9. The reason to use a set is
 * to avoid counting twice a point that can be reached through multiple paths. The problem statement
 * asks for the sizes of the three largest basins so we check the size of the set and take the
 * highest three.
 */

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
    let map = Map::new(map);

    let low_points: Vec<(usize, (usize, usize))> = map.low_points();

    let part_1: usize = low_points.iter().map(|(n, _)| n + 1).sum();
    println!("Part 1: {}", part_1);

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|(_, coords)| map.basin_size(*coords))
        .collect();
    basin_sizes.sort_by_key(|n| -((*n) as isize));
    let part_2: usize = basin_sizes[0..3].iter().product();
    println!("Part 2: {}", part_2);
}

struct Map {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(map: Vec<Vec<usize>>) -> Self {
        let height = map.len();
        let width = map.first().unwrap().len();

        Map { map, width, height }
    }

    fn low_points(&self) -> Vec<(usize, (usize, usize))> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, n)| {
                        if x > 0 && self.map[y][x - 1] <= *n {
                            None
                        } else if y > 0 && self.map[y - 1][x] <= *n {
                            None
                        } else if x < (self.width - 1) && self.map[y][x + 1] <= *n {
                            None
                        } else if y < (self.height - 1) && self.map[y + 1][x] <= *n {
                            None
                        } else {
                            Some((*n, (x, y)))
                        }
                    })
                    .collect::<Vec<(usize, (usize, usize))>>()
            })
            .collect()
    }

    fn basin_size(&self, coords: (usize, usize)) -> usize {
        self.basin_map(coords).len()
    }

    fn basin_map(&self, (x, y): (usize, usize)) -> HashSet<(usize, usize)> {
        let mut result = HashSet::from_iter([(x, y)]);
        let n = self.map[y][x];

        if x > 0 && self.map[y][x - 1] > n && self.map[y][x - 1] != 9 {
            result.extend(self.basin_map((x - 1, y)));
        }

        if y > 0 && self.map[y - 1][x] > n && self.map[y - 1][x] != 9 {
            result.extend(self.basin_map((x, y - 1)));
        }

        if x < (self.width - 1) && self.map[y][x + 1] > n && self.map[y][x + 1] != 9 {
            result.extend(self.basin_map((x + 1, y)));
        }

        if y < (self.height - 1) && self.map[y + 1][x] > n && self.map[y + 1][x] != 9 {
            result.extend(self.basin_map((x, y + 1)));
        }

        result
    }
}
