use std::fs;
use regex::Regex;

fn ways_to_win(time: f64, dist: f64) -> i64 {
    let root1: f64 = (time - (time * time - 4.0 * dist).sqrt()) / 2.0;
    (time - 2.0 * root1.floor() - 1.0) as i64
}
fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    for part in 1..=2 {
        let numbers: Vec<i64> = re.find_iter(&input)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let n = numbers.len() / 2;
        let result: i64 = (0..n)
            .map(|i| ways_to_win(numbers[i] as f64, numbers[i + n] as f64))
            .product();
        println!("Part {}: {}", part, result);
        input.retain(|c| c != ' ');
    }
}
