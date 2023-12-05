use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input.txt");

    let scores: Vec<u32> = input
        .lines()
        .map(|line| {
            let mut card_numbers = line
                .split(':')
                .nth(1).expect("card numbers")
                .split('|');

            let winning: HashSet<u32> = card_numbers
                .nth(0).expect("winning numbers")
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("parsable number"))
                .collect();
            let ours: HashSet<u32> = card_numbers
                .nth(0).expect("our numbers")
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("parsable number"))
                .collect();

            let winners: Vec<u32> = winning.intersection(&ours).copied().collect();
            winners
        })
        .collect::<Vec<Vec<u32>>>()
        .iter()
        .filter(|nums| nums.len() > 0)
        .map(|nums| score(nums.len() as u32))
        .collect();

    let sum: u32 = scores.iter().sum();
    println!("{sum}");
}

fn score(number: u32) -> u32 {
    u32::pow(2, number - 1)
}

