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

    let numbers: Vec<usize> = file
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let part_1 = part_1(numbers.clone());
    println!("Part 1 {:?}", part_1);

    let part_2 = part_2(numbers);
    println!("Part 2 {:?}", part_2);
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

fn part_2(numbers: Vec<usize>) -> usize {
    let max = numbers.iter().max().unwrap();

    (0..=*max)
        .map(|position| {
            numbers
                .iter()
                .map(|n| {
                    let difference = (position as isize - *n as isize).abs() as usize;
                    difference * (difference + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
