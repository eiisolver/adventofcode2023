use std::collections::HashMap;
use std::fs;

fn calc_steps(nodes: &Vec<[usize; 2]>, directions: &Vec<usize>, is_end: &Vec<bool>, start_node: usize) -> usize {
    let mut count = 0;
    let mut node = start_node;
    while !is_end[node] {
        node = nodes[node][directions[count % directions.len()]];
        count += 1;
    }
    count
}

fn main() {
    let mut node_map = HashMap::new();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut is_end: Vec<bool> = Vec::new();
    let mut part2_start_nodes: Vec<usize> = Vec::new();
    for (index, line) in input.lines().skip(2).enumerate() {
        let label = line.get(0..3).unwrap();
        node_map.insert(label, index);
        is_end.push(label.chars().nth(2).unwrap() == 'Z');
        if label.chars().nth(2).unwrap() == 'A' {
            part2_start_nodes.push(index);
        }
    }
    let mut nodes: Vec<_> = Vec::new();
    for line in input.lines().skip(2) {
        let mut arr: [usize; 2] = [0; 2];
        arr[0] = node_map.get(line.get(7..10).unwrap()).unwrap().clone();
        arr[1] = node_map.get(line.get(12..15).unwrap()).unwrap().clone();
        nodes.push(arr);
    }
    let directions: Vec<usize> = input.lines().nth(0).unwrap().chars()
            .map(|c| if c == 'L' { 0 } else {1})
            .collect();
    let part1 = calc_steps(&nodes, &directions, &is_end, *node_map.get("AAA").unwrap());
    println!("Part 1: {}", part1);
    
    // Manual inspection reveals: every start node reaches only 1 end node,
    // with a periodicity that is a prime * 263. So total periodicity is the
    // product of these primes * 263.
    let mut part2: u128 = 1;
    for start in part2_start_nodes {
        part2 *= calc_steps(&nodes, &directions, &is_end, start) as u128;
        part2 /= 263;
    }
    println!("Part 2: {}", part2 * 263);
}
