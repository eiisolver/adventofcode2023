use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
struct Part {
    row: i32,
    start: i32,
    end: i32,
    value: i128,
}

impl Part {
    fn is_adj(&self, row: i32, col: i32) -> bool {
        return self.start - 1 <= col && self.end >= col 
                && self.row - 1 <= row && self.row + 1 >= row;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let H = lines.len();
    let W = lines[0].len();
    let mut adj = Vec::with_capacity(H);
    for line in &lines {
        let mut r: Vec<bool> = line.chars().map(|c| false).collect();
        adj.push(r);
    }
    for r in 0..H {
        for c in 0..W {
            let v: char = lines[r].chars().nth(c).unwrap();
            if v != '.' && !v.is_digit(10) {
                for r2 in r-1..r+2 {
                    for c2 in c-1..c+2 {
                        if r2 < H && c2 < W {
                            adj[r2][c2] = true;
                        }
                    }
                }
            }
        }
    }
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut r = 0;
    let mut sum: i128 = 0;
    let mut parts: Vec<Part> = Vec::new();
    for line in &lines {
        for m in re.find_iter(line) {
            let nr: i128 = m.as_str().parse().unwrap();
            let slice = &adj[r][m.start()..m.end()];
            if slice.iter().any(|&x| x) {
                sum += nr;
            }
            let part = Part {
                row: r as i32,
                start: m.start() as i32,
                end: m.end() as i32,
                value: nr
            };
            parts.push(part);
        }
        r += 1;
    }
    println!("Part 1: {}", sum);
    
    let mut sum2: i128 = 0;
    for r in 0..H {
        for c in 0..W {
            let v: char = lines[r].chars().nth(c).unwrap();
            if v == '*' {
                let mut v: i128 = 1;
                let mut nr_adj = 0;
                for part in &parts {
                    if part.is_adj(r as i32, c as i32) {
                        nr_adj += 1;
                        v *= part.value;
                    }
                }
                if nr_adj == 2 {
                    sum2 += v;
                }
            }
        }
    }
    println!("Part 2: {}", sum2);
    Ok(())
}
