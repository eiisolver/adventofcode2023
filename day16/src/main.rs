use std::fs;
use std::time::Instant;

const N: usize = 0;
const E: usize = 1;
const S: usize = 2;
const W: usize = 3;
static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
static MIRROR_0: [usize; 4] = [E, N, W, S]; // / mirror
static MIRROR_1: [usize; 4] = [W, S, E, N]; // \ mirror
static SPLIT_VER_0: [usize; 4] = [N, N, S, N]; // | splitter, primary dir
static SPLIT_VER_1: [Option<usize>; 4] = [None, Some(S), None, Some(S)]; // | splitter, secondary
static SPLIT_HOR_0: [usize; 4] = [E, E, E, W]; // - splitter, primary dir
static SPLIT_HOR_1: [Option<usize>; 4] = [Some(W), None, Some(W), None]; // - splitter, secndary

struct Grid {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
    next_node: Vec<(Option<usize>, Option<usize>)>,  // Precalculated neighbours
    visited: Vec<bool>,
    splits: Vec<usize>, // Start nodes of spawned beams
}
impl Grid {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let mut g = Grid {
            grid: grid.clone(),
            w: grid[0].len(),
            h: grid.len(),
            next_node: Vec::new(),
            visited: Vec::new(),
            splits: Vec::new(),
        };
        g.visited = vec![false; g.w * g.h * 4];
        for node in 0..g.visited.len() {
            g.next_node.push(g.next(node));
        }
        g
    }
    fn next(&self, node: usize) -> (Option<usize>, Option<usize>) {
        let (row, col, dir) = self.from_node(node);
        let (new_dir, split_dir) = match self.grid[row as usize][col as usize] {
            '.' => (dir, None),
            '/' => (MIRROR_0[dir], None),
            '\\' => (MIRROR_1[dir], None),
            '|' => (SPLIT_VER_0[dir], SPLIT_VER_1[dir]),
            '-' => (SPLIT_HOR_0[dir], SPLIT_HOR_1[dir]),
            _ => (dir, None)
        };
        let next_node = self.to_node(row + DIRS[new_dir].0, col + DIRS[new_dir].1, new_dir);
        let next_split_node: Option<usize> = if let Some(d) = split_dir {
            self.to_node(row + DIRS[d].0, col + DIRS[d].1, d)
        } else {
            None
        };
        (next_node, next_split_node)
    }
    fn from_node(&self, node: usize) -> (i32, i32, usize) {
        ((node / (4 * self.w)) as i32, ((node / 4) % self.w) as i32,  node % 4)
    }
    fn to_node(&self, row: i32, col: i32, dir: usize) -> Option<usize> {
        if row < 0 || row >= self.w as i32 || col < 0 || col >= self.h as i32 {
            None
        } else {
            Some(4 * ((row as usize) * self.w + (col as usize)) + dir)
        }
    }
    fn beam(&mut self, node: usize) {
        let mut n = node;
        while !self.visited[n] {
            self.visited[n] = true;
            let (nxt, nxt_split) = self.next_node[n];
            if let Some(split) = nxt_split {
                self.splits.push(split); // Beam is split into 2 directions, spawn new beam
            }
            if let Some(next_n) = nxt {
                n = next_n;
            } else {
                break; // Next node is off the grid
            }
        }
    }
    fn beam_and_split(&mut self, row: usize, col: usize, dir: usize) -> usize {
        let n = self.to_node(row as i32, col as i32, dir).unwrap();
        self.splits.clear();
        self.visited.fill(false);
        self.beam(n);
        for ix in 0.. {
            if ix >= self.splits.len() {
                break;
            }
            self.beam(self.splits[ix]);
        }
        self.visited.chunks(4).map(|c| c.iter().any(|v| *v)).filter(|v| *v).count()
    }
    fn part2(&mut self) -> usize {
        let mut results = Vec::new();
        for col in 0..self.w {
            results.push(self.beam_and_split(0, col, S));
            results.push(self.beam_and_split(self.h - 1, col, N));
        }
        for row in 0..self.h {
            results.push(self.beam_and_split(row, 0, E));
            results.push(self.beam_and_split(row, self.w - 1, W));
        }
        results.iter().max().unwrap().clone()
    }
}

fn main() {
    let start_time = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let mut g = Grid::new(&grid);
    println!("Part 1: {}", g.beam_and_split(0, 0, E));
    println!("Part 2: {}", g.part2());
    println!("Took: {:?}", start_time.elapsed());
}
