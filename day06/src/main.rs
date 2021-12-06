use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

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

    // Each lanternfish is completely independent from the rest in terms of how many offspring it
    // will produce, so we can calculate it separately for each of them and add them up.
    let part_1: usize = initial_state
        .iter()
        .map(|starting_timer| population_simulation(*starting_timer, 80))
        .sum();

    println!("Part 1: {}", part_1);
}

fn population_simulation(starting_timer: usize, simulation_days: usize) -> usize {
    // Each lanternfish will have offspring the day *after* the timer runs out, so it will only have
    // offspring if there are enough simulation days for the timer to finish *and* one more to reset
    let offspring = if simulation_days >= starting_timer + 1 {
        (simulation_days - (starting_timer + 1)) / DAYS_UNTIL_OFFSPRING + 1
    } else {
        0
    };

    // The grand total of the simulation for one lanternfish is the total for each of its offspring
    // plus itself.
    (1..=offspring)
        .map(|n| {
            population_simulation(
                starting_timer + 2 + n * DAYS_UNTIL_OFFSPRING,
                simulation_days,
            )
        })
        .sum::<usize>()
        + 1
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
