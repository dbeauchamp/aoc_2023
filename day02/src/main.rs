#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    blue: Option<usize>,
    green: Option<usize>,
    red: Option<usize>,
}

fn main() {
    let input = include_str!("../input/input.txt");

    let games: Vec<Game> = input
        .lines()
        .map(|l| parse_game(l))
        .collect();

    let mut powers: Vec<usize> = Vec::new();
    for game in games {

        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for set in game.sets {
            if let Some(blue) = set.blue {
                if blue > max_blue {
                    max_blue = blue;
                }
            }
            if let Some(green) = set.green {
                if green > max_green {
                    max_green = green;
                }
            }
            if let Some(red) = set.red {
                if red > max_red {
                    max_red = red;
                }
            }
        }

        powers.push(max_blue * max_red * max_green);
    }

    let sum: usize = powers.iter().sum();

    println!("{sum}");
}

fn parse_game(input: &str) -> Game {
    let mut split = input.split(':');
    let id = split
        .next()
        .expect("game exists")
        .split_whitespace()
        .nth(1)
        .expect("id exists")
        .parse::<usize>()
        .unwrap();


    let sets: Vec<Set> = split
        .next()
        .expect("sets")
        .split(';')
        .map(|s| build_set(s))
        .collect();

    Game {
        id,
        sets,
    }
}

fn build_set(input: &str) -> Set {
    let blocks: Vec<&str> = input
        .split(',')
        .collect();

    let mut blue = None;
    let mut green = None;
    let mut red = None;
    for b in blocks {
        let mut split = b.split_whitespace();
        let num: usize = split.next().expect("number of blocks").parse().unwrap();
        let color = split.next().expect("color");
        match color {
            "blue" => blue = Some(num),
            "green" => green = Some(num),
            "red" => red = Some(num),
            _ => panic!("couldn't parse colors")
        }
    }

    Set {
        blue,
        green,
        red,
    }
}
