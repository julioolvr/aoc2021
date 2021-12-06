use cached::proc_macro::cached;
use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

/**
 * --- Day 6: Lanternfish ---
 *
 * The program has to simulate a growing population of lanternfish, where each lanternfish spawns a
 * new one every 7 days, the starting population is at different stages of its cycle, and the newly
 * spawned lanternfishes have to wait an extra 2 days before spawning its first offspring.
 *
 * Each lanternfish is completely independent from the rest, so `population_simulation` does all the
 * calculation based only on the starting timer for a given lanternfish (i.e. how many days until
 * the next spawn) and the remaining simulation days.
 *
 * The trick to make it scale for part two is memoizing that function and choosing the parameters
 * in a way that will repeat often. One way to think about it is that, except for the lanternfishes
 * on the input file, every single one after that will always have a starting_timer of 8. So the
 * only parameter that really changes are the remaining simulation days - which will always
 * decrease. In part 2 the number of days is 256, so the function will be called at most 256 * 8 =
 * 2,048 times. After that it will always use memoized results so it finishes pretty quickly
 * (~3.2ms on my machine).
 */

const DAYS_UNTIL_OFFSPRING: usize = 7;

fn main() {
    let initial_state: Vec<usize> = read_lines()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let part_1: usize = initial_state
        .iter()
        .map(|starting_timer| population_simulation(*starting_timer, 80))
        .sum();

    println!("Part 1: {}", part_1);

    let part_2: usize = initial_state
        .iter()
        .map(|starting_timer| population_simulation(*starting_timer, 256))
        .sum();

    println!("Part 2: {}", part_2);
}

#[cached]
fn population_simulation(starting_timer: usize, simulation_days: usize) -> usize {
    // Each lanternfish will have offspring the day *after* the timer runs out, so it will only have
    // offspring if there are enough simulation days for the timer to finish *and* one more to
    // reset.
    let offspring = if simulation_days >= starting_timer + 1 {
        // We subtract the starting_timer (+ 1 for the extra day needed for the first offspring to
        // spawn) from the simulation_days to get rid of that first, and dividing the remaining
        // simulation days by 7 will give us how many more offspring will spawn. Because of that
        // initial subtraction we're not counting the very first offspring, so we add it to the
        // result.
        (simulation_days - (starting_timer + 1)) / DAYS_UNTIL_OFFSPRING + 1
    } else {
        0
    };

    // The grand total of the simulation for one lanternfish is the total for each of its offspring
    // plus itself.
    (0..offspring)
        .map(|n| {
            simulation_days
                // The remaining simulation days for each offspring is the starting simulation days
                // of its parent, minus the days until its parent started producing offspring, minus
                // 7 days for each subsequent offspring (and one extra day for the initial hatch).
                // We start the range at 0 above so the first offspring will only wait for the
                // (starting_timer + 1).
                .checked_sub(starting_timer + 1 + n * DAYS_UNTIL_OFFSPRING)
                .map(|remaining_simulation_days| {
                    population_simulation(8, remaining_simulation_days)
                })
                .unwrap_or(1)
        })
        .sum::<usize>()
        + 1
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
