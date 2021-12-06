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
 * on the input file, every single one after that will always days_until_next_offspring of 9. So the
 * only parameter that really changes are the remaining simulation days - which will always
 * decrease. In part 2 the number of days is 256, so the function will be called at most 256 * 8 =
 * 2,048 times. After that it will always use memoized results so it finishes pretty quickly
 * (~3.2ms on my machine).
 */

const CYCLE_LENGTH: usize = 7;
const EXTRA_DAYS_FOR_FIRST_OFFSPRING: usize = 2;

fn main() {
    let initial_state: Vec<usize> = read_lines()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    // One thing to note on the kickoff call of population_simulation for each lanternfish is that
    // the offspring spawns the day _after_ starting_timer gets down to 0. That means that for a
    // starting_timer of 3, it will actually be 4 days until a new offspring spawns. That's why
    // each of these calls have to add 1 to starting_timer.

    let part_1: usize = initial_state
        .iter()
        .map(|starting_timer| population_simulation(*starting_timer + 1, 80))
        .sum();

    println!("Part 1: {}", part_1);

    let part_2: usize = initial_state
        .iter()
        .map(|starting_timer| population_simulation(*starting_timer + 1, 256))
        .sum();

    println!("Part 2: {}", part_2);
}

#[cached]
fn population_simulation(days_until_next_offspring: usize, simulation_days: usize) -> usize {
    let offspring = if simulation_days >= days_until_next_offspring {
        // We subtract the days_until_next_offspring from the simulation_days to cover the first
        // offspring, so dividing the remaining simulation days by 7 will give us how many more
        // offspring will spawn. Because of that initial subtraction we're not counting the very
        // first offspring, so we add 1 to the result.
        (simulation_days - days_until_next_offspring) / CYCLE_LENGTH + 1
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
                // We start the range at 0 above so the first offspring will only wait for
                // days_until_next_offspring.
                .checked_sub(days_until_next_offspring + n * CYCLE_LENGTH)
                .map(|remaining_simulation_days| {
                    population_simulation(
                        CYCLE_LENGTH + EXTRA_DAYS_FOR_FIRST_OFFSPRING,
                        remaining_simulation_days,
                    )
                })
                // If there aren't enough days for a full cycle, we just count the offspring itself.
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
