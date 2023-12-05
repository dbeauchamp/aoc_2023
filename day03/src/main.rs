#[derive(Debug)]
struct PartNumber {
    number: u32,
    start: (i32, i32),
    length: u32,
    seen: bool,
}

fn main() {
    let input = include_str!("../input/input.txt");

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        part_numbers.append(&mut extract_part_numbers(i, line));
    }

    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '*' => {
                    // search through each part_number
                    let curr = (j as i32, i as i32);
                    let mut adjacent: Vec<u32> = Vec::new();
                    for pn in &mut part_numbers {
                        if pn.seen { continue; }

                        let range_x = (pn.start.0 - 1)..(pn.start.0 + 1 + pn.length as i32);
                        let range_y = (curr.1 - 1)..=(curr.1 + 1);
                        if range_x.contains(&curr.0) &&  range_y.contains(&pn.start.1) {
                            pn.seen = true;
                            adjacent.push(pn.number);
                        }
                    }

                    if adjacent.len() == 2 {
                        sum = sum + (adjacent[0] * adjacent[1]);
                    }
                },
                _ => continue,
            }
        }
    }

    println!("{sum}");
}

fn extract_part_numbers(row: usize, line: &str) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    let mut start = None;
    for (i, c) in line.chars().enumerate() {
        match c {
            '0'..='9' => {
                if start.is_none() {
                    start = Some(i);
                }
               if i == line.chars().count() - 1 {
                    let n = start.unwrap();
                    let part_number = PartNumber {
                        number: (&line[n..=i]).parse::<u32>().unwrap(),
                        start: (n as i32, row as i32),
                        length: (i - n + 1) as u32,
                        seen: false,
                    };
                    part_numbers.push(part_number);
                }
            }
            _ => {
                if let Some(n) = start {
                    let part_number = PartNumber {
                        number: (&line[n..i]).parse::<u32>().unwrap(),
                        start: (n as i32, row as i32),
                        length: (i - n) as u32,
                        seen: false,
                    };
                    part_numbers.push(part_number);
                }
                start = None;
            }
        }
    }

    part_numbers
}
