use std::fs;

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    // Convert cards to hex
    input = input.replace('A', "E").replace('K', "D").replace('Q', "C").replace('J', "B").replace('T', "A");
    for part in 1..=2 {
        let mut hands: Vec<_> = Vec::new(); // vector of tuple (type, hand, bid)
        for line in input.lines() {
            let tok: Vec<&str> = line.split(" ").collect();
            let hand = usize::from_str_radix(&tok[0], 16).unwrap();
            let mut count: [u32; 16] = [0; 16];
            for i in 0..5 {
                count[(hand >> (4 * i)) & 0xf] += 1;
            }
            let mut jokers = 0;
            if part == 2 {
                jokers = count[1];
                count[1] = 0;
            }
            count.sort();
            count.reverse();
            count[0] += jokers;
            let bid: usize = tok[1].parse().unwrap();
            hands.push((10 * count[0] + count[1], hand, bid));
        }
        hands.sort();
        let result: usize = hands.iter().enumerate().map(|(index, hand)| (index + 1) * hand.2).sum();
        println!("Part {}: {}", part, result);
        input = input.replace('B', "1"); // Convert J to a 1 for part 2
    }
}
