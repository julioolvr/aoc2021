use std::{collections::HashSet, env, iter::FromIterator, ops::Index};

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

type Coordinates = (usize, usize);

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

    fn low_points(&self) -> Vec<(usize, Coordinates)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, n)| {
                        if self
                            .neighbors((x, y))
                            .iter()
                            .all(|(neighbor, _)| neighbor > n)
                        {
                            Some((*n, (x, y)))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(usize, Coordinates)>>()
            })
            .collect()
    }

    fn basin_size(&self, coords: Coordinates) -> usize {
        self.basin_map(coords).len()
    }

    fn basin_map(&self, coords: Coordinates) -> HashSet<Coordinates> {
        let n = self[coords];
        self.neighbors(coords)
            .iter()
            .filter(|(neighbor, _)| *neighbor > n && *neighbor != 9)
            .fold(HashSet::from_iter([coords]), |mut set, (_, (x, y))| {
                set.extend(self.basin_map((*x, *y)));
                set
            })
    }

    fn neighbors(&self, (x, y): Coordinates) -> Vec<(usize, Coordinates)> {
        let mut result = vec![];

        if x > 0 {
            result.push((self[(x - 1, y)], (x - 1, y)));
        }

        if y > 0 {
            result.push((self[(x, y - 1)], (x, y - 1)));
        }

        if x < (self.width - 1) {
            result.push((self[(x + 1, y)], (x + 1, y)));
        }

        if y < (self.height - 1) {
            result.push((self[(x, y + 1)], (x, y + 1)));
        }

        return result;
    }
}

impl Index<Coordinates> for Map {
    type Output = usize;

    fn index(&self, (x, y): Coordinates) -> &Self::Output {
        &self.map[y][x]
    }
}
