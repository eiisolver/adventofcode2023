use std::fs;

static DIRS: [(i128, i128); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn shoelace_area(points: &Vec<(i128, i128)>) -> i128 {
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        sum += x1 * y2 - x2 * y1;
    }
    (sum / 2).abs()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    for part in 1..=2 {
        let mut instr: Vec<_> = Vec::new();
        for line in input.lines() {
            let mut toks = line.split(' ');
            let mut dir = DIRS["RDLU".find(toks.next().unwrap()).unwrap()];
            let mut amount: i128 = toks.next().unwrap().parse().unwrap();
            let color_str = toks.next().unwrap();
            let color = u32::from_str_radix(&color_str[2..8], 16).unwrap();
            if part == 2 {
                dir = DIRS[(color & 0x3) as usize];
                amount = (color >> 4) as i128;
            }
            instr.push((dir, amount, color));
        }
        let mut point = (0, 0);
        let mut points = Vec::new();
        points.push((0, 0));
        let mut amount_sum = 0;
        for ((dr, dc), amount, _) in &instr {
            amount_sum += *amount;
            point = (point.0 + amount * dr, point.1 + amount * dc);
            points.push(point);
        }
        let result = shoelace_area(&points) + 1 + amount_sum / 2;
        println!("Part {part}: {result}");
    }
}
