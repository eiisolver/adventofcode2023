use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug)]
struct Cond {
    part: String,
    op: String,
    value: usize,
    nxt: String,
}

#[derive(Debug)]
enum Rule {
    Check(Cond),
    Done(String),
}

type Part = HashMap<String, usize>;
// For part 2: range of values
type PartRange = HashMap<String, (usize, usize)>;

// Evaluates a rule, returns name of next station in the workflow
fn eval<'a>(rule: &'a Vec<Rule>, part: &'a Part) -> &'a str {
    for cond in rule {
        match cond {
            Rule::Check(c) => {
                let part_value = part.get(&c.part).unwrap();
                let result;
                if &c.op == "<" {
                    result = *part_value < c.value;
                } else if &c.op == ">" {
                    result = *part_value > c.value;
                } else {
                    panic!("Unsupported operator");
                }
                if result {
                    return &c.nxt;
                }
            }
            Rule::Done(s) => {
                return s;
            }
        }
    }
    "A"
}

// Evaluates a part for a rule, returns vector (key of next, part range).
fn eval_range(curr_key: &str, rule: &Vec<Rule>, part: &PartRange) -> Vec<(String, PartRange)> {
    let mut result: Vec<(String, PartRange)> = Vec::new();
    for cond in rule {
        match cond {
            Rule::Check(c) => {
                let (part_from, part_to) = part.get(&c.part).unwrap();
                if &c.op == "<" {
                    if *part_to <= c.value {
                        // Entire range fits -> move it to next station
                        result.push((c.nxt.clone(), part.clone()));
                        break;
                    } else if *part_from < c.value {
                        // Range must be split
                        let (p1, p2) = split_part(part, &c.part, c.value);
                        result.push((curr_key.to_string(), p1));
                        result.push((curr_key.to_string(), p2));
                        break;
                    }
                } else {
                    // >
                    if *part_from > c.value {
                        // Entire range fits
                        result.push((c.nxt.clone(), part.clone()));
                        break;
                    } else if *part_to - 1 > c.value {
                        // Range must be split
                        let (p1, p2) = split_part(part, &c.part, c.value + 1);
                        result.push((curr_key.to_string(), p1));
                        result.push((curr_key.to_string(), p2));
                        break;
                    }
                }
            }
            Rule::Done(s) => {
                result.push((s.clone(), part.clone()));
                break;
            }
        }
    }
    result
}

// Split a part range in 2 sub ranges
fn split_part(part: &PartRange, key: &str, value: usize) -> (PartRange, PartRange) {
    let mut part1 = HashMap::new();
    let mut part2 = HashMap::new();
    for (k, (from, to)) in part.iter() {
        if k == key {
            part1.insert(k.clone(), (from.clone(), value));
            part2.insert(k.clone(), (value, to.clone()));
        } else {
            part1.insert(k.clone(), (from.clone(), to.clone()));
            part2.insert(k.clone(), (from.clone(), to.clone()));
        }
    }
    (part1, part2)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (workflow_str, rating_str) = input.split_once("\n\n").unwrap();

    // Create workflow
    let mut workflow: HashMap<&str, Vec<Rule>> = HashMap::new();
    for line in workflow_str.lines() {
        let (key, rule_str) = line[0..line.len() - 1].split_once('{').unwrap();
        let mut rules = Vec::new();
        for t in rule_str.split(',') {
            let rule = if t.contains(":") {
                let (c, key2) = t.split_once(':').unwrap();
                let cond = Cond {
                    part: c[0..1].to_string(),
                    op: c[1..2].to_string(),
                    value: c[2..].parse().unwrap(),
                    nxt: key2.to_string(),
                };
                Rule::Check(cond)
            } else {
                Rule::Done(t.to_string())
            };
            rules.push(rule);
        }
        workflow.insert(key, rules);
    }

    // Create parts
    let mut parts: Vec<Part> = Vec::new();
    for line in rating_str.lines() {
        let mut r: Part = HashMap::new();
        for p in line[1..line.len() - 1].split(',') {
            let (part, v) = p.split_once('=').unwrap();
            r.insert(part.to_string(), v.parse().unwrap());
        }
        parts.push(r);
    }
    // Part 1: process the parts
    let mut result = 0;
    for part in parts {
        let mut key = "in";
        while key != "A" && key != "R" {
            key = eval(workflow.get(key).unwrap(), &part);
        }
        if key == "A" {
            result += part.iter().map(|(_, v)| *v).sum::<usize>();
        }
    }
    println!("Part 1: {result}");

    // Part 2
    let mut q: VecDeque<(String, PartRange)> = VecDeque::new();
    let initial_range = (
        "in".to_string(),
        HashMap::from([
            ("x".to_string(), (1, 4001)),
            ("m".to_string(), (1, 4001)),
            ("a".to_string(), (1, 4001)),
            ("s".to_string(), (1, 4001)),
        ]),
    );
    q.push_back(initial_range);
    let mut result2: usize = 0;
    while !q.is_empty() {
        let (k, part) = q.pop_front().unwrap();
        if k == "A" {
            // Accepted
            result2 += part.iter().map(|(_, (fr, to))| to - fr).product::<usize>();
        } else if k == "R" {
            // Rejected
        } else {
            // Add result(s) to the queue
            let nxt = eval_range(&k, workflow.get(k.as_str()).unwrap(), &part);
            for p in nxt {
                q.push_back(p);
            }
        }
    }
    println!("Part 2: {result2}");
}
