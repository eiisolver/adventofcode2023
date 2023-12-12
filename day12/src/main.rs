use std::collections::HashMap;
use std::fs;

type Cache = HashMap<usize, usize>;

fn arrange(cond: &Vec<char>, groups: &Vec<usize>, cond_ix: usize, grp_ix: usize, cache: &mut Cache) -> usize {
    if cond_ix >= cond.len() {
        return if grp_ix == groups.len() {1} else {0};
    }
    let key = 10000 * cond_ix + grp_ix;
    let value = cache.get(&key);
    if value.is_some() {
        return value.unwrap().clone();
    }
    let c = cond[cond_ix];
    let mut result: usize = 0;
    if c == '.' || c == '?' {
        result += arrange(cond, groups, cond_ix + 1, grp_ix, cache);
    }
    if (c == '#' || c == '?') && grp_ix < groups.len() {
        let len = groups[grp_ix];
        if len + cond_ix <= cond.len() {
            if !cond[cond_ix..cond_ix + len].iter().any(|c| *c == '.') {
                if len + cond_ix == cond.len() {
                    result += arrange(cond, groups, cond_ix + len, grp_ix + 1, cache);
                } else if cond[cond_ix + len] != '#' {
                    result += arrange(cond, groups, cond_ix + len + 1, grp_ix + 1, cache);
                }
            }
        }
    }
    cache.insert(key, result);
    result
}

fn arrangements(cond: &Vec<char>, groups: &Vec<usize>) -> usize {
    let mut cache: Cache = HashMap::new();
    return arrange(cond, groups, 0, 0, &mut cache);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let mut t = line.split(" ");
        let cond = t.next().unwrap().chars().collect();
        let groups: Vec<usize> = t.next().unwrap().split(",").map(|c|c.parse().unwrap()).collect();
        part1 += arrangements(&cond, &groups);
        let mut cond2: Vec<char> = Vec::new();
        let mut groups2 = Vec::new();
        for i in 0..5 {
            cond2.extend(cond.clone());
            if i < 4 {
                cond2.push('?');
            }
            groups2.extend(groups.clone());
        }
        part2 += arrangements(&cond2, &groups2);
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
