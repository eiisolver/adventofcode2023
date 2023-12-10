use std::fs;

static DIRS: [i32; 8] = [-1, 0, 0, 1, 1, 0, 0, -1];
static OPP_DIR: [usize; 4] = [2, 3, 0, 1];

type Pos = [i32; 2];
type Grid = Vec<Vec<usize>>;

fn next_pos(grid: &Grid, pos: &Pos, in_dir: usize) -> (Pos, usize) {
    let mut v = grid[pos[0] as usize][pos[1] as usize];
    let mask =  1 << OPP_DIR[in_dir];
    if v & mask == 0 {
        return ([-1, -1], 4);
    }
    v &= !mask;
    let dir = v.trailing_zeros() as usize;
    let row = pos[0] + DIRS[2 * dir];
    let col = pos[1] + DIRS[2 * dir + 1];
    ([row, col], dir)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = Vec::new();
    let mut start_pos: Pos = [0; 2];
    for line in input.lines() {
        let col: Vec<usize> = line.chars()
                .map(|c| {
                    match c {
                        '.' => 0,
                        'L' => 3,
                        '|' => 5,
                        'J' => 9,
                        'F' => 6,
                        '7' => 12,
                        '-' => 10,
                        'S' => 15,
                        _ => 0
                    }
                }).collect();
        let start_col = col.iter().position(|v| *v == 15);
        if start_col.is_some() {
            start_pos = [grid.len() as i32, start_col.unwrap() as i32];
        }
        grid.push(col);
    }
    let mut pos: Pos = [start_pos[0], start_pos[1] + 1];
    let mut dir: usize = 1;
    let mut dist: u32 = 1;
    let mut path = Vec::new();
    path.push(start_pos.clone());
    while pos != start_pos {
        path.push(pos.clone());
        (pos, dir) = next_pos(&grid, &pos, dir);
        dist += 1;
    }
    println!("Part 1: {}", dist / 2);

    path.push(start_pos.clone());
    let h = 2 * grid.len();
    let w = 2 * grid[0].len();
    let mut grid2 = Vec::new();
    for _ in 0..h {
        grid2.push(vec![0; w]);
    }
    for i in 0..path.len() {
        let r: usize = 2 * path[i][0] as usize;
        let c: usize = 2 * path[i][1] as usize;
        grid2[r][c] = 2;
        if i < path.len() - 1 {
            let r2 = 2 * path[i+1][0] as usize;
            let c2 = 2 * path[i+1][1] as usize;
            grid2[(r + r2)/2][(c + c2)/2] = 2;
        }
    }
    let mut updated = true;
    while updated {
        updated = false;
        for r in 0..h {
            for c in 0..w {
                if grid2[r][c] != 0 {
                    continue
                }
                if r == 0 || r == h-1 || c == 0 || c == w-1 {
                    grid2[r][c] = 1;
                    updated = true;
                } else {
                    for d in 0..4 {
                        let r2 = r.wrapping_add_signed(DIRS[2*d] as isize);
                        let c2 = c.wrapping_add_signed(DIRS[2*d + 1] as isize);
                        if grid2[r2][c2] == 1 {
                            grid2[r][c] = 1;
                            updated = true;
                        }
                    }
                }
            }
        }
    }
    let mut part2: usize = 0;
    for r in 0..h/2 {
        for c in 0..w/2 {
            if grid2[2*r][2*c] == 0 {
                part2 += 1;
            }
        }
    }
    println!("Part 2: {}", part2);
}
