use std::fs;

struct Box {
    lenses: Vec<(String, usize)>,
    id: usize,
}

impl Box {
    pub fn new(id: usize) -> Self {
        Self {
            lenses: Vec::new(),
            id: id,
        } 
    }
    fn remove(&mut self, label: &str) {
        match self.find(label) {
            Some(index) => {self.lenses.remove(index); ()},
            _ => ()
        }
    }
    fn replace(&mut self, label: &str, focal_length: usize) {
        match self.find(label) {
            Some(index) => self.lenses[index] = (label.to_string(), focal_length),
            _ => self.lenses.push((label.to_string(), focal_length))
        }
    }
    fn power(&self) -> usize {
        let mut pow: usize = 0;
        for (ix, (_, len)) in self.lenses.iter().enumerate() {
            pow += (ix + 1) * len;
        }
        (self.id + 1) * pow
    }
    fn find(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|(s, _)| (*s).as_str() == label)
    }
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| 17 * (acc + (c as usize))) & 0xff
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let instr: Vec<&str> = input.split(",").collect();
    let part1: usize = instr.iter().map(|s| hash(s)).sum();
    println!("Part 1: {}", part1);
    let mut boxes: Vec<Box> = (0..256).map(|id| Box::new(id)).collect();
    for s in instr {
        let mut tok = s.split(&['-', '=']);
        let label = tok.next().unwrap();
        let bx = hash(label);
        let par = tok.next().unwrap();
        if par.is_empty() {
            boxes[bx].remove(label);
        } else {
            boxes[bx].replace(label, par.parse().unwrap());
        }
    }
    let part2: usize = boxes.iter().map(|b| b.power()).sum();
    println!("Part 2: {}", part2);
}
