use std::fs;

fn is_hor_mirror(area: &Vec<Vec<char>>, index: usize) -> bool {
    let mut ix = index;
    let mut ix2 = index + 1;
    let mut equal = true;
    while equal {
        equal = area[ix] == area[ix2];
        if ix == 0 || ix2 >= area.len() - 1 {
            break;
        }
        ix -= 1;
        ix2 += 1;
    }
    equal
}
fn find_hor_mirror(area: &Vec<Vec<char>>, skip: usize) -> Option<usize> {
    let mut result = None;
    for i in 0..area.len() - 1 {
        if i+1 != skip && is_hor_mirror(area, i) {
            result = Some(i+1);
        }
    }
    result
}
fn find_mirror(area: &Vec<Vec<char>>, part1_result: usize) -> usize {
    let hor = find_hor_mirror(area, part1_result / 100);
    let result = 100 * hor.unwrap_or(0);
    let mut flipped: Vec<Vec<char>> = Vec::new();
    for i in 0..area[0].len() {
        let mut s = Vec::new();
        for j in 0..area.len() {
            s.push(area[j][i]);
        }
        flipped.push(s);
    }
    result +  find_hor_mirror(&flipped, part1_result % 100).unwrap_or(0)
}
fn find_smudge(area: &mut Vec<Vec<char>>, part1_result: usize) -> usize {
    for i in 0..area.len() {
        for j in 0..area[i].len() {
            let c = area[i][j];
            area[i][j] = if c == '.' {'#'} else {'.'};
            let result = find_mirror(&area, part1_result);
            if result > 0 && part1_result != result {
                return result;
            }
            area[i][j] = c;
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut area: Vec<Vec<char>> = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        if line.len() == 0 {
            let result1 = find_mirror(&area, 0);
            part1 += result1;
            part2 += find_smudge(&mut area, result1);
            area.clear();
        } else {
            area.push(line.chars().collect());
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}