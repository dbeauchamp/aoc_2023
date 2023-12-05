use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input.txt");

    let scores: Vec<usize> = input
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
            winners.len()
        })
        .collect();

    let mut counts: Vec<usize> = vec![1; scores.len()];
    for (i, score) in scores.iter().enumerate() {
        for j in i+1..=(i+score) {
            counts[j] += 1*counts[i];
        }
    }

    let sum: usize = counts.iter().sum();
    println!("{sum}");
}

fn score(number: u32) -> u32 {
    u32::pow(2, number - 1)
}

