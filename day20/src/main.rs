use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
struct Module {
    name: String,
    module: char,
    state: usize,
    receivers: Vec<(usize, usize)>, // receiver index,
    inputs: Vec<usize>,             // for conjunction: inputs
}
#[derive(Debug)]
struct Pulse {
    value: usize,    // 0 or 1
    receiver: usize, // Receiver index
    input: usize,    // INput index of receiver
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut modules = Vec::new();

    // Create modules
    for line in input.lines() {
        let (p1, _) = line.split_once(" -> ").unwrap();
        let name = &p1[1..];
        map.insert(name.to_string(), modules.len());
        modules.push(Module {
            name: name.to_string(),
            module: p1.chars().next().unwrap(),
            state: 0,
            receivers: Vec::new(),
            inputs: Vec::new(),
        });
    }
    // Calculate receivers/inputs of modules
    for (ix, line) in input.lines().enumerate() {
        let (_, p2) = line.split_once(" -> ").unwrap();
        for name in p2.split(", ") {
            if let Some(ix2) = map.get(name) {
                let input_ix = modules[*ix2].inputs.len();
                modules[ix].receivers.push((*ix2, input_ix));
                modules[*ix2].inputs.push(0);
            } else {
                // Add "sink" (module that is only output)
                let len = modules.len();
                modules[ix].receivers.push((len, 0));
                map.insert(name.to_string(), len);
                modules.push(Module {
                    name: name.to_string(),
                    module: '.',
                    state: 0,
                    receivers: Vec::new(),
                    inputs: Vec::new(),
                });
            }
        }
    }

    // Output graph representation in dot format
    println!("digraph day20 {{");
    for m in &modules {
        let mut s = "circle";
        if m.module == '%' {
            s = "box";
        } else if m.module == '&' {
            s = "diamond";
        }
        println!("  {} [shape={}]", m.name, s);
    }
    for m in &modules {
        if !m.receivers.is_empty() {
            print!("  {} -> {{", m.name);
            for (r, _) in &m.receivers {
                print!("{} ", modules[*r].name);
            }
            println!("}}");
        }
    }
    println!("}}");

    // Part 1 (and 2) calculation
    let rx = map.get("rx").unwrap();
    let mut rx_pulses = 0;
    let mut q = VecDeque::new();
    let mut count: [usize; 2] = [0; 2];
    for x in 0..20000 {
        if x == 1000 {
            println!("Part 1: {}", count[0] * count[1]);
        }
        if rx_pulses == 1 {
            println!("Part 2: rx pulses after {x}: {rx_pulses}");
            if rx_pulses == 1 {
                break;
            }
        }
        rx_pulses = 0;

        q.push_back(Pulse {
            value: 0,
            receiver: *map.get("roadcaster").unwrap(),
            input: 0,
        });
        while !q.is_empty() {
            let pulse = q.pop_front().unwrap();
            count[pulse.value] += 1;
            let mut send = false;
            let mut value = 0;
            {
                let m = &mut modules[pulse.receiver];
                match m.module {
                    'b' => {
                        // broadcaster
                        send = true;
                        value = 0;
                    }
                    '%' => {
                        if pulse.value == 0 {
                            m.state = 1 - m.state;
                            send = true;
                            value = m.state;
                        }
                    }
                    '&' => {
                        send = true;
                        let ix = pulse.input.clone();
                        m.inputs[ix] = pulse.value.clone();
                        if m.inputs.iter().sum::<usize>() == m.inputs.len() {
                            value = 0;
                            if m.inputs.len() > 1 {
                                println!("{}: con {}", x + 1, m.name);
                            }
                        } else {
                            value = 1;
                        }
                    }
                    _ => (),
                }
                if pulse.receiver == *rx {
                    rx_pulses += 1;
                }
            }
            let m2: &Module = &modules[pulse.receiver];
            if send {
                for (receiver, input) in &m2.receivers {
                    q.push_back(Pulse {
                        value,
                        receiver: *receiver,
                        input: *input,
                    });
                }
            }
        }
    }
    // Part 2 manually computed by multiplying the outputs (prime numbers) for the
    // conjuction modules.
    // Part 2: 211712400442661
}
