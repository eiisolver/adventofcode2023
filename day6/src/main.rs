use num_integer::Roots;
use std::fs;
use regex::Regex;

fn ways_to_win(time: i64, dist: i64) -> i64 {
    let v = 4 * time * time - 16 * (dist + 1);
    time % 2 + v.sqrt() / 2
}
fn main() {
    let mut input = fs::read_to_string("test_input.txt").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    for part in 1..=2 {
        let numbers: Vec<i64> = re.find_iter(&input)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let n = numbers.len() / 2;
        let result: i64 = (0..n)
            .map(|i| ways_to_win(numbers[i], numbers[i + n]))
            .product();
        println!("Part {}: {}", part, result);
        input.retain(|c| c != ' ');
    }
}
