use std::env;

/**
 * --- Day 7: The Treachery of Whales ---
 *
 * The program takes as input a list of initial positions for a bunch of crabs in submarines. The
 * positions are made out of a single value each, because the crabs only move in one axis. The
 * purpose of both parts is to find what's the best position to put all crabs in to optimize fuel
 * consumption, where how to calculate "fuel consumption" changes in part 1 and part 2.
 *
 * For part 1, moving 1 step costs 1 fuel. So the challenge is to find the target position that
 * minimizes the distance to each position in the input list. Instead of checking each possible
 * target we can calculate the *mode* of the input list. This is the value that splits the list in
 * half. This is the position that minimizes the distance - moving one position to the side will, in
 * the best case, add 1 to the distance for every crab in one half and subtract one from the other
 * half, ending up in the same total fuel consumption. Worst case, moving to the side will make a
 * crab from one half cross to the other half, so if we keep moving the target in that direction the
 * total distance will start to increase.
 *
 * For part 2, moving costs 1 extra fuel per step. So moving 1 step costs 1 fuel, moving 2 costs
 * 1+2=3 fuel, moving 3 costs 1+2+3=6 fuel and so on. It might be possible to do something fancier
 * but this implementation simply tries each position and calculates the total fuel cost of moving
 * all crabs there. Looking at the input file the max position is less than 2,000 and there are
 * 1,000 numbers, so the total is a little bit under 2,000,000 iterations which finishes pretty
 * quickly. Gauss sum is used to calculate the fuel cost of moving from some position to some target
 * in order to avoid yet another nesting level of loops.
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

    let positions: Vec<usize> = file
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let part_1 = part_1(&positions);
    println!("Part 1 {:?}", part_1);

    let part_2 = part_2(&positions);
    println!("Part 2 {:?}", part_2);
}

fn part_1(positions: &[usize]) -> usize {
    // Calculate the mode for the set of numbers
    let mut numbers = positions.to_vec();
    numbers.sort();

    let mode = if numbers.len() % 2 == 0 {
        (numbers[numbers.len() / 2 - 1] + numbers[numbers.len() / 2]) / 2
    } else {
        numbers[numbers.len() / 2]
    };

    // Calculate fuel needed to get there
    numbers
        .iter()
        .map(|n| (mode as isize - *n as isize).abs() as usize)
        .sum()
}

fn part_2(positions: &[usize]) -> usize {
    let max = positions.iter().max().unwrap();

    (0..=*max)
        .map(|target| {
            positions
                .iter()
                .map(|n| {
                    let difference = (target as isize - *n as isize).abs() as usize;
                    difference * (difference + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
