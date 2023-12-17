use pathfinding::prelude::dijkstra;
use std::fs;
use std::time::Instant;

static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
static OPP_DIR: [usize; 4] = [2, 3, 0, 1];

fn to_node(row: usize, col: usize, dir: usize, w: usize) -> usize {
    4 * w * row + 4 * col + dir
}

#[allow(dead_code)]
fn from_node(node: usize, w: usize) -> (usize, usize, usize) {
    let row = node / (4 * w);
    let col = (node / 4) % w;
    (row, col, node % 4)
}

fn main() {
    let start_time = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|c| (c as u32) - 48).collect());
    }
    let h = grid.len();
    let w = grid[0].len();
    for part in 1..=2 {
        let mut graph: Vec<Vec<(usize, u32)>> = Vec::new(); // vector (neighbour, cost)
        let range = if part == 1 { (1, 3) } else { (4, 10) };
        // Create graph
        for row in 0..h {
            for col in 0..w {
                for dir in 0..4 {
                    let n = to_node(row, col, dir, w);
                    let mut edges = Vec::new();
                    for dir2 in 0..4 {
                        if n != 0 && (dir2 == dir || dir2 == OPP_DIR[dir]) {
                            continue;
                        }
                        let mut cost = 0;
                        for step in 1..=range.1 {
                            let row2: i32 = (row as i32) + step * DIRS[dir2].0;
                            let col2: i32 = (col as i32) + step * DIRS[dir2].1;
                            if row2 >= 0 && row2 < h as i32 && col2 >= 0 && col2 < w as i32 {
                                cost += grid[row2 as usize][col2 as usize];
                                if step >= range.0 {
                                    let n2 = to_node(row2 as usize, col2 as usize, dir2, w);
                                    edges.push((n2, cost));
                                }
                            }
                        }
                    }
                    graph.push(edges);
                }
            }
        }
        // Run dijkstra
        let start: usize = 0;
        let result2 = dijkstra(&start, |p| graph[*p].clone(), |p| *p >= 4 * w * h - 4);
        println!("Part {part}: {:?}", result2.unwrap().1);
    }
    println!("Took: {:?}", start_time.elapsed());
}
