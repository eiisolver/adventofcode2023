use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

fn main() {
    let re = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    let reader = BufReader::new(File::open("2_input.txt").expect("Cannot open file"));
    let mut count = 0;
    let mut power_sum: i64 = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let mut matches: Vec<_> = re.find_iter(s.as_ref()).map(|m| m.as_str()).collect();
        let game_id: i32 = matches[1].parse().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut i = 0;
        let mut amount = 0;
        for tok in matches.split_off(2) {
            if i == 0 {
                amount = tok.parse().unwrap();
            } else {
                match tok {
                    "red" => red = std::cmp::max(red, amount),
                    "green" => green = std::cmp::max(green, amount),
                    "blue" => blue = std::cmp::max(blue, amount),
                    _ => println!("?"),
                }
            }
            i = 1 - i;
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            count += game_id;
        }
        power_sum += red * green * blue;

    }
    println!("Part 1: {}", count);
    println!("Part 2: {}", power_sum);
}
