use std::fs;

fn north(grid: &mut Vec<Vec<char>>) {
    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != 'O' {
                continue;
            }
            for m in (0..i).rev() {
                if grid[m][j] == '#' {
                    break;
                }
                if grid[m][j] == '.' {
                    grid[m][j] = 'O';
                    grid[m+1][j] = '.';
                }
            }
        }
    }
}
fn south(grid: &mut Vec<Vec<char>>) {
    for i in (0..grid.len()-1).rev() {
        for j in 0..grid[i].len() {
            if grid[i][j] != 'O' {
                continue;
            }
            for m in i+1..grid.len() {
                if grid[m][j] == '#' {
                    break;
                }
                if grid[m][j] == '.' {
                    grid[m][j] = 'O';
                    grid[m-1][j] = '.';
                }
            }
        }
    }
}
fn west(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 1..grid[i].len() {
            if grid[i][j] != 'O' {
                continue;
            }
            for m in (0..j).rev() {
                if grid[i][m] == '#' {
                    break;
                }
                if grid[i][m] == '.' {
                    grid[i][m] = 'O';
                    grid[i][m+1] = '.';
                }
            }
        }
    }
}
fn east(grid: &mut Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in (0..grid[i].len()-1).rev() {
            if grid[i][j] != 'O' {
                continue;
            }
            for m in j+1..grid[i].len() {
                if grid[i][m] == '#' {
                    break;
                }
                if grid[i][m] == '.' {
                    grid[i][m] = 'O';
                    grid[i][m-1] = '.';
                }
            }
        }
    }
}

fn calc_load(grid: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                load += grid.len() - i;
            }
        }
    }
    load
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let mut grid2 = grid.clone();
    north(&mut grid);
    

    println!("Part 1: {}", calc_load(&grid));
    let mut hist: Vec<Vec<Vec<char>>> = Vec::new();
    let mut round: usize = 0;
    let n: usize = 1000000000;
    while round < n {
        hist.push(grid2.clone());
        north(&mut grid2);
        west(&mut grid2);
        south(&mut grid2);
        east(&mut grid2);
        round += 1;
        for j in (0..hist.len()).rev() {
            if grid2 == hist[j] {
                let period = round - j;
                if (n - round) % period == 0 {
                    println!("Part 2: {}", calc_load(&grid2));
                    return;
                }
            }
        }
    }
}
