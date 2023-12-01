fn main() {
    let input = include_str!("../input/input.txt");

    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|l| parse_values(l))
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("{sum}");
}

fn parse_values(line: &str) -> u32 {
    let mut digits: String = line
        .chars()
        .filter(|c| c.is_digit(10))
        .into_iter()
        .collect();

    let num = if digits.len() == 1 {
        digits.clone() + &digits
    } else {
        [digits.remove(0), digits.remove(digits.len() - 1)].iter().collect()
    };

    num.parse::<u32>().unwrap()
}
