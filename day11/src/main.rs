use std::collections::HashSet;
use itertools::Itertools;
use std::fs;

fn get_map(set: &HashSet<i32>, expansion: &i32) -> Vec<i32> {
    let max = set.iter().max().unwrap().clone();
    let mut v = 0;
    let mut result = Vec::new();
    for i in 0..=max {
        result.push(v);
        if set.contains(&i) {
            v += 1;
        } else {
            v += expansion;
        }
    }
    return result;
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    let mut h: i32 = 0;
    for line in input.lines() {
        let cols: Vec<i32> = line.chars().enumerate()
                .filter(|(_, b)| *b == '#')
                .map(|(a, _)| a as i32).collect();
        for c in cols {
            galaxies.push((h, c));
        }
        h += 1;
    }
    let mut used_cols = HashSet::new();
    let mut used_rows = HashSet::new();
    for (r, c) in &galaxies {
        used_rows.insert(*r);
        used_cols.insert(*c);
    }
    for part in 1..=2 {
        let expansion = if part == 1 {2} else {1000000};
        let row_map = get_map(&used_rows, &expansion);
        let col_map = get_map(&used_cols, &expansion);
        let mut result: i64 = 0;
        for ((r1, c1), (r2, c2)) in galaxies.iter().tuple_combinations() {
            result += ((row_map[*r1 as usize] - row_map[*r2 as usize]).abs() 
                    + (col_map[*c1 as usize] - col_map[*c2 as usize]).abs()) as i64;
        }
        println!("Part {}: {}", part, result);
    }
}