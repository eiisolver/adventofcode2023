use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Brick {
    pos: Vec<i32>,
    id: usize,
}

impl Brick {
    fn lowest_z(&self) -> i32 {
        return self.pos[2];
    }
    // Return this brick's cubes at z == given z.
    fn cubes(&self, z: i32) -> Vec<(i32, i32, i32)> {
        let mut result = Vec::new();
        let zd = z - self.lowest_z();
        for x in self.pos[0]..=self.pos[3] {
            for y in self.pos[1]..=self.pos[4] {
                for zp in self.pos[2]..=self.pos[5] {
                    result.push((x, y, zp + zd));
                }
            }
        }
        return result;
    }
}
fn main() {
    // Read input
    let re = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut bricks = Vec::new();
    for (id, line) in input.lines().enumerate() {
        let pos: Vec<i32> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        bricks.push(Brick { pos, id });
    }
    // Sort bricks
    bricks.sort_by(|a, b| a.pos[2].cmp(&b.pos[2]));

    // Let the bricks fall
    let mut cubes: HashMap<(i32, i32, i32), usize> = HashMap::new();
    let mut highest_z: i32 = 0;
    for (i, b) in bricks.iter().enumerate() {
        for z in (1..=highest_z + 1).rev() {
            let brick_cubes = b.cubes(z);
            let mut will_stick = false;
            for c in &brick_cubes {
                let c_under = (c.0, c.1, c.2 - 1);
                if c.2 <= 1 || cubes.contains_key(&c_under) {
                    will_stick = true;
                    break;
                }
            }
            if will_stick {
                highest_z = highest_z.max(brick_cubes.iter().map(|c| c.2).max().unwrap());
                for c in brick_cubes {
                    cubes.insert(c, i);
                }
                break;
            }
        }
    }
    // Determine relations between bricks
    let mut rests_on: Vec<HashSet<usize>> = Vec::new();
    let mut supports: Vec<HashSet<usize>> = Vec::new();
    for _ in &bricks {
        rests_on.push(HashSet::new());
        supports.push(HashSet::new());
    }
    for (c, i) in &cubes {
        let c2 = (c.0, c.1, c.2 + 1);
        let j = cubes.get(&c2);
        if let Some(k) = j {
            if *k != *i {
                rests_on[*k].insert(*i);
                supports[*i].insert(*k);
            }
        }
    }
    // Calculate bricks that cannot be disintegrated
    let mut required_bricks = HashSet::new();
    for rest_set in &rests_on {
        if rest_set.len() == 1 {
            let i = rest_set.iter().next();
            if let Some(k) = i {
                required_bricks.insert(*k);
            }
        }
    }
    println!("Part 1: {}", bricks.len() - required_bricks.len());
    // Part 2: for each required brick calc the closure of other bricks that would fall
    let mut total_disintegrated: usize = 0;
    for i in &required_bricks {
        let mut disintegrated = HashSet::new();
        disintegrated.insert(i);
        loop {
            let mut added = Vec::new();
            for &j in &disintegrated {
                for candidate in &supports[*j] {
                    if disintegrated.contains(candidate) {
                        continue;
                    }
                    let mut falls = true;
                    for r in &rests_on[*candidate] {
                        // Candidate rests on a non-disintegrated brick and will not fall
                        if !disintegrated.contains(r) {
                            falls = false;
                        }
                    }
                    if falls {
                        added.push(candidate);
                    }
                }
            }
            if added.is_empty() {
                break;
            }
            for k in added {
                disintegrated.insert(k);
            }
        }
        total_disintegrated += disintegrated.len() - 1;
    }
    println!("Part 2: {total_disintegrated}");
}
