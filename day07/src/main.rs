fn main() {
    let numbers: Vec<usize> = include_str!("../input.txt")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let part_1 = part_1(numbers);
    println!("Part 1 {:?}", part_1);
}

fn part_1(mut numbers: Vec<usize>) -> usize {
    // Calculate the mode for the set of numbers
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
