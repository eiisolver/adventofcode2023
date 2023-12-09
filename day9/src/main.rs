use std::fs;

fn predict(input: &Vec<i64>) -> i64 {
    if input.iter().all(|c| *c == 0) {
        0
    } else {
        let diff: Vec<i64> = input.iter().skip(1)
            .scan(input[0], |state, &x| {
                let d: i64 = x - *state;
                *state = x;
                Some(d)
            }).collect();
        predict(&diff) + input.last().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    for line in input.lines() {
        let mut nrs: Vec<i64> = line.split(" ").map(|c| c.parse().unwrap()).collect();
        part1 += predict(&nrs);
        nrs.reverse();
        part2 += predict(&nrs);
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
