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
    let mut nums = Vec::new();
    let mut word = "".to_string();
    for c in line.chars() {
        if c.is_digit(10) {
            word = "".to_string();
            nums.push(c.to_digit(10).unwrap());
            continue;
        }

        word.push(c);
        if let Some(num) = word_to_number(&word) {
            // incase the last letter of a word is the beginning of next word
            word = word.pop().unwrap().to_string();
            nums.push(num);
        }
    }

    nums[0]*10 + nums.last().unwrap()
}

fn word_to_number(word: &str) -> Option<u32> {
    if word.contains("one") {
        return Some(1);
    }
    if word.contains("two") {
        return Some(2);
    }
    if word.contains("three") {
        return Some(3);
    }
    if word.contains("four") {
        return Some(4);
    }
    if word.contains("five") {
        return Some(5);
    }
    if word.contains("six") {
        return Some(6);
    }
    if word.contains("seven") {
        return Some(7);
    }
    if word.contains("eight") {
        return Some(8);
    }
    if word.contains("nine") {
        return Some(9);
    }

    None
}
