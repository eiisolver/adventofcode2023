use std::{collections::HashSet, fs};

static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
static STEPS: usize = 26501365; // == 202300*131 + 65
static BABY_STEPS: usize = 2 * 131 + 65; // Same properties as STEPS, but smaller...

fn count_sq(r: i32, c: i32, n: i32, squares: &HashSet<(i32, i32)>) -> usize {
    let r0 = r * n;
    let c0 = c * n;
    let mut result = 0;
    for (row, col) in squares.iter() {
        if *row >= r0 && *row < r0 + n && *col >= c0 && *col < c0 + n {
            result += 1;
        }
    }
    result
}

fn count_sq_4(r: i32, c: i32, n: i32, squares: &HashSet<(i32, i32)>) -> usize {
    count_sq(r, c, n, squares)
        + count_sq(-r, c, n, squares)
        + count_sq(r, -c, n, squares)
        + count_sq(-r, -c, n, squares)
}

// Extrapolate results for STEPS based on results for BABY_STEPS.
fn part2(n: usize, squares: &HashSet<(i32, i32)>) {
    let a = (STEPS - 130) / 131;
    let b = (a + 1) / 2;
    let odd = 4 * b * b;
    let even = 2 * a * (a + 1) + 1 - odd;
    let n0 = n as i32;
    let full = odd * count_sq(0, 1, n0, squares) + even * count_sq(0, 0, n0, squares);
    let diag = a * count_sq_4(1, 1, n0, squares) + (a + 1) * count_sq_4(2, 1, n0, squares);

    let top = count_sq(2, 0, n0, squares)
        + count_sq(0, 2, n0, squares)
        + count_sq(-2, 0, n0, squares)
        + count_sq(0, -2, n0, squares);
    println!("Part 2: {}", full + diag + top);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut walkable: HashSet<(i32, i32)> = HashSet::new();
    let mut start = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' || c == 'S' {
                walkable.insert((row as i32, col as i32));
            }
            if c == 'S' {
                start = (row as i32, col as i32);
            }
        }
    }

    let mut squares: HashSet<(i32, i32)> = HashSet::new();
    squares.insert(start);
    for x in 0..64 {
        let mut new_squares: HashSet<(i32, i32)> = HashSet::new();
        for (row, col) in squares {
            for (dr, dc) in DIRS {
                let (r2, c2) = (row + dr, col + dc);
                if walkable.contains(&(r2, c2)) {
                    new_squares.insert((r2, c2));
                }
            }
        }
        squares = new_squares;
    }
    println!("Part 1: {}", squares.len());
    let n = input.lines().count();

    squares.clear();
    squares.insert(start);
    for x in 1..=BABY_STEPS {
        let mut new_squares: HashSet<(i32, i32)> = HashSet::new();
        for (row, col) in squares {
            for (dr, dc) in DIRS {
                let (r2, c2) = (row + dr, col + dc);
                let sq2 = (r2.rem_euclid(n as i32), c2.rem_euclid(n as i32));
                if walkable.contains(&sq2) {
                    new_squares.insert((r2, c2));
                }
            }
        }
        squares = new_squares;
        if x == BABY_STEPS {
            part2(n, &squares);
        }
    }
}
