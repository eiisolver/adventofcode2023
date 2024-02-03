use std::{cmp, collections::HashMap, fs};

static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
// y, x
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    pub fn dir(&self, d: usize) -> Self {
        Pos(self.0 + DIRS[d].0, self.1 + DIRS[d].1)
    }
    pub fn neighbours(&self) -> [Pos; 4] {
        [self.dir(0), self.dir(1), self.dir(2), self.dir(3)]
    }
    pub fn in_grid(&self, rows: usize, cols: usize) -> bool {
        self.0 >= 0 && self.0 < rows as i32 && self.1 >= 0 && self.1 < cols as i32
    }
}

fn can_goto(dir: usize, c: char) -> bool {
    match c {
        '.' => true,
        '>' => dir == 1,
        '<' => dir == 3,
        '^' => dir == 0,
        'v' => dir == 2,
        _ => false,
    }
}

static mut BEST: usize = 0;

fn walk(
    depth: usize,
    pos: &Pos,
    end: &Pos,
    graph: &HashMap<Pos, Vec<Pos>>,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let r = pos.0 as usize;
    let c = pos.1 as usize;
    if visited[r][c] {
        return 0;
    }
    if pos == end {
        unsafe {
            if depth > BEST {
                BEST = depth;
                println!("Best: {depth}");
            }
        }
        return depth;
    }
    let mut result = 0;
    visited[r][c] = true;
    for pos2 in graph.get(pos).unwrap() {
        let result2 = walk(depth + 1, &pos2, end, graph, visited);
        result = cmp::max(result, result2);
    }
    visited[r][c] = false;
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let h = grid.len();
    let w = grid[0].len();
    let start = Pos(0, 1);
    let end: Pos = Pos(h as i32 - 1, w as i32 - 2);
    let mut graph: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut graph2: HashMap<Pos, Vec<Pos>> = HashMap::new();
    for (r, row) in grid.iter().enumerate() {
        for c in 0..row.len() {
            let pos = Pos(r as i32, c as i32);
            let mut nb = Vec::new();
            let mut nb2 = Vec::new();
            for (d, pos2) in pos.neighbours().iter().enumerate() {
                if pos2.in_grid(grid.len(), grid[0].len()) {
                    let c2 = grid[pos2.0 as usize][pos2.1 as usize];
                    if can_goto(d, c2) {
                        nb.push(pos2.clone());
                    }
                    if c2 != '#' {
                        nb2.push(pos2.clone());
                    }
                }
            }
            graph.insert(pos.clone(), nb);
            graph2.insert(pos, nb2);
        }
    }
    let mut visited = vec![vec![false; w]; h];
    println!("Part1: {}", walk(0, &start, &end, &graph, &mut visited));
    println!("Part2: {}", walk(0, &start, &end, &graph2, &mut visited));
}
