use itertools::Itertools;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use substring::Substring;

fn floodfill(graph: &Vec<Vec<usize>>, src: usize, dist: &mut Vec<usize>, node_name: &Vec<&str>) {
    for i in 0..dist.len() {
        dist[i] = 1000000;
    }
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    vec1.push(src);
    let mut d = 0;
    dist[src] = 0;
    while !vec1.is_empty() {
        d += 1;
        for n in &vec1 {
            //println!("Node {}", node_name[*n]);
            for n2 in &graph[*n] {
                if dist[*n2] > d {
                    dist[*n2] = d;
                    vec2.push(*n2);
                    //println!("  Depth {d}: {}", node_name[*n2]);
                }
            }
        }
        vec1.clear();
        std::mem::swap(&mut vec1, &mut vec2);
    }
}
fn main() {
    let mut name_to_node: HashMap<&str, usize> = HashMap::new();
    let mut node_name: Vec<&str> = Vec::new();
    let mut graph: Vec<Vec<usize>> = Vec::new();

    // Read input
    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let mut names: Vec<_> = line.split(' ').collect();
        names[0] = names[0].substring(0, 3);
        for (i, name) in names.iter().enumerate() {
            let node: usize = match name_to_node.get(name) {
                None => {
                    let n = node_name.len();
                    name_to_node.insert(name, n);
                    node_name.push(name);
                    graph.push(Vec::new());
                    n
                }
                Some(n) => *n,
            };
            if i > 0 {
                let n0 = name_to_node[names[0]];
                graph[n0].push(node);
                graph[node].push(n0);
            }
        }
    }
    // Find 2 nodes far from each other, do a floodfill
    let mut dist: Vec<usize> = vec![0; graph.len()];
    floodfill(&graph, 0, &mut dist, &node_name);
    let src = dist.iter().position_max().unwrap();
    println!(
        "zero: {}, src: {}, dist: {}",
        node_name[0], node_name[src], dist[src]
    );
    floodfill(&graph, src, &mut dist, &node_name);
    let sink = dist.iter().position_max().unwrap();
    println!(
        "Source: {}, sink: {}, dist: {}",
        node_name[src], node_name[sink], dist[sink]
    );
    let src = sink;
    floodfill(&graph, src, &mut dist, &node_name);
    let sink = dist.iter().position_max().unwrap();
    println!(
        "Source: {}, sink: {}, dist: {}",
        node_name[src], node_name[sink], dist[sink]
    );

    // After manually removing the 3 correct edges, the graph is cut in 2.
    let count1: usize = dist.iter().map(|v| if *v < 1000 { 1 } else { 0 }).sum();
    let count2 = graph.len() - count1;
    println!("Count1: {count1}, count2: {count2}");
    println!("Part 1: {}", count1 * count2);

    // Following code helped to find these 3 edges.
    // I solved the puzzle in a manual assisted way:
    // walk 10000 random walks between 2 nodes far from each other,
    // count which edges are most often visited.
    // Those that appear often, and are far away from the source/sink are likely
    // candidates, so I removed these from the input, and reran.
    let mut edge_count = vec![0; graph.len() * graph.len()];
    for _ in 0..10000 {
        let mut n = sink;
        while n != src {
            let mut best_n2 = 0;
            let mut best_v: usize = 0;
            for n2 in &graph[n] {
                if dist[*n2] < dist[n] {
                    let r: usize = rand::random::<usize>() % 10000;
                    if r > best_v {
                        best_v = r;
                        best_n2 = *n2;
                    }
                }
            }
            let edge = if n < best_n2 {
                n * graph.len() + best_n2
            } else {
                best_n2 * graph.len() + n
            };
            edge_count[edge] += 1;
            n = best_n2;
        }
    }
    for i in 0..10 {
        let e = edge_count.iter().position_max().unwrap();
        let n = e % graph.len();
        let n2 = e / graph.len();
        println!(
            "{i}: {}-{}, count {}, dist: {}",
            node_name[n], node_name[n2], edge_count[e], dist[n]
        );
        edge_count[e] = 0;
    }
    //ptq -- fxn
}
