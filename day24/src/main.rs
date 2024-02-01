use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Obj {
    pos: [i128; 3],
    v: [i128; 3],
    a: f64,
    b: f64,
}
impl Obj {
    fn in_future(&self, x: f64) -> bool {
        if x >= self.pos[0] as f64 {
            self.v[0] > 0
        } else {
            self.v[0] < 0
        }
    }
}
fn main() {
    //let limits: [i128; 2] = [7, 27];
    let limits: [i128; 2] = [200000000000000, 400000000000000];
    let re = Regex::new(r"[\-0-9]+").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();
    let mut objs: Vec<Obj> = Vec::new();
    for line in input.lines() {
        let mut nrs = Vec::new();
        for m in re.find_iter(line) {
            let nr: i128 = m.as_str().parse().unwrap();
            nrs.push(nr);
        }
        println!("{:?}", nrs);
        let x: f64 = nrs[0] as f64;
        let y: f64 = nrs[1] as f64;
        let vx: f64 = nrs[3] as f64;
        let vy: f64 = nrs[4] as f64;
        let a: f64 = vy / vx;
        let b: f64 = y - (x / vx) * vy;
        let obj = Obj {
            pos: [nrs[0], nrs[1], nrs[2]],
            v: [nrs[3], nrs[4], nrs[5]],
            a: a,
            b: b,
        };
        println!(" f(x) = {a}*x + {b}, at x={x}: {}", a * x + b);
        objs.push(obj);
    }
    let mut count = 0;
    for i in 0..objs.len() {
        for j in i + 1..objs.len() {
            let x = (objs[j].b - objs[i].b) / (objs[i].a - objs[j].a);
            if x >= (limits[0] as f64)
                && x <= (limits[1] as f64)
                && objs[i].in_future(x)
                && objs[j].in_future(x)
            {
                let y = objs[i].a * x + objs[i].b;
                if y >= (limits[0] as f64) && y <= (limits[1] as f64) {
                    count += 1;
                }
            }
        }
    }
    println!("Part 1: {count}");
}
