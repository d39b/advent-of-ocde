use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let dw = parse_input("example.txt");
    let dw = parse_input("input.txt");
    // let dw = parse_input("input_fixed.txt");
    part1(&dw);
    part2(&dw);
}

fn part1(dw: &DeviceWiring) {
    let mut values = vec![false; dw.n];
    let mut has_value = vec![false; dw.n];
    for &(i, v) in dw.initial_values.iter() {
        values[i] = v;
        has_value[i] = true;
    }

    let mut in_q = vec![false; dw.gates.len()];
    let mut adj = vec![vec![]; dw.n];
    let mut q = vec![];
    for (i, g) in dw.gates.iter().enumerate() {
        let a;
        let b;
        match g {
            &Gate::AND(x, y, _) => {
                a = x;
                b = y;
            }
            &Gate::OR(x, y, _) => {
                a = x;
                b = y;
            }
            &Gate::XOR(x, y, _) => {
                a = x;
                b = y;
            }
        }

        if has_value[a] && has_value[b] {
            q.push(i);
            in_q[i] = true;
        }

        adj[a].push(i);
        adj[b].push(i);
    }

    let mut qi = 0;
    while qi < q.len() {
        let gate = &dw.gates[q[qi]];
        qi += 1;
        let c;
        match gate {
            &Gate::AND(x, y, z) => {
                c = z;
                values[z] = values[x] && values[y];
            }
            &Gate::OR(x, y, z) => {
                c = z;
                values[z] = values[x] || values[y];
            }
            &Gate::XOR(x, y, z) => {
                c = z;
                values[z] = values[x] ^ values[y];
            }
        }
        has_value[c] = true;
        for &gi in adj[c].iter() {
            let a;
            let b;
            match &dw.gates[gi] {
                &Gate::AND(x, y, _) => {
                    a = x;
                    b = y;
                }
                &Gate::OR(x, y, _) => {
                    a = x;
                    b = y;
                }
                &Gate::XOR(x, y, _) => {
                    a = x;
                    b = y;
                }
            }

            if has_value[a] && has_value[b] {
                q.push(gi);
                in_q[gi] = true;
            }
        }
    }

    let mut result = 0usize;
    for (i, &v) in dw.outputs.iter().enumerate() {
        if values[v] {
            result |= 1 << i;
        }
    }
    println!("{result}");
}

fn part2(dw: &DeviceWiring) {
    // we have 44 bits in both inputs x and y
    // output z has 45 bits
    // there are ~200 gates
    // for the swaps we have to pick 8 gates
    // 200 over 8 ~ 5x10^13 so we cannot simulate all possibilities
    // there is probably some structure in the gates that we can exploit
    // so for every bit we compute x_i ^ y_i    and x_i & y_i
    // and for each of those we get a variable name, lets call it xor_i and and_i
    // and the results go into variables unless for i = 0
    // for every bit position there is one wire that represents the carry
    // initially that is just x00 & y00 (it is 1 only if both are 1)
    // for the output we have z_i = (x_i ^ y_i) ^ carry_(i-1) = xor_i ^ carry_(i-1) (makes sense)
    // we also do b = xor_i & carry_(i-1) = (x_i ^ y_i) & carry_(i-1)
    // and then we compute carry_i = b or and_i
    // does that carry make sense? it is set if both x_i and y_i are 1, or if only
    // one bit of x_i and y_i is set and the previous carry was set -> sounds about right
    // so we know what the circuit should look like, but how can we easily find the switched
    // gate outputs?
    // just check the result of some simple inputs
    // e.g. we can set just two bits x_i and y_i compute the output and check what goes wrong
    for i in 0..45 {
        for a in 0..2 {
            let x = a << i;
            for b in 0..2 {
                let y = b << i;
                let r = run_device(dw, x, y);
                if r != x + y {
                    println!(
                        "wrong result for bit {i}, {a} + {b} got {r} expected {}",
                        x + y
                    );
                }
            }
        }
    }
    // from this output we can find the mistakes by hand
    // z07 has been swapped with nqk
    // pcp has been swapped with fgt
    // z24 has been swapped with fpq
    // z32 has been swapped with srn
    // sorted we get fgt,fpq,nqk,pcp,srn,z07,z24,z32
}

fn run_device(dw: &DeviceWiring, x: usize, y: usize) -> usize {
    let mut values = vec![false; dw.n];
    let mut has_value = vec![false; dw.n];
    for i in 0..45 {
        if x & (1 << i) > 0 {
            values[i] = true;
        }
        has_value[i] = true;
    }
    for i in 0..45 {
        if y & (1 << i) > 0 {
            values[i + 45] = true;
        }
        has_value[i + 45] = true;
    }

    let mut in_q = vec![false; dw.gates.len()];
    let mut adj = vec![vec![]; dw.n];
    let mut q = vec![];
    for (i, g) in dw.gates.iter().enumerate() {
        let a;
        let b;
        match g {
            &Gate::AND(x, y, _) => {
                a = x;
                b = y;
            }
            &Gate::OR(x, y, _) => {
                a = x;
                b = y;
            }
            &Gate::XOR(x, y, _) => {
                a = x;
                b = y;
            }
        }

        if has_value[a] && has_value[b] {
            q.push(i);
            in_q[i] = true;
        }

        adj[a].push(i);
        adj[b].push(i);
    }

    let mut qi = 0;
    while qi < q.len() {
        let gate = &dw.gates[q[qi]];
        qi += 1;
        let c;
        match gate {
            &Gate::AND(x, y, z) => {
                c = z;
                values[z] = values[x] && values[y];
            }
            &Gate::OR(x, y, z) => {
                c = z;
                values[z] = values[x] || values[y];
            }
            &Gate::XOR(x, y, z) => {
                c = z;
                values[z] = values[x] ^ values[y];
            }
        }
        has_value[c] = true;
        for &gi in adj[c].iter() {
            let a;
            let b;
            match &dw.gates[gi] {
                &Gate::AND(x, y, _) => {
                    a = x;
                    b = y;
                }
                &Gate::OR(x, y, _) => {
                    a = x;
                    b = y;
                }
                &Gate::XOR(x, y, _) => {
                    a = x;
                    b = y;
                }
            }

            if has_value[a] && has_value[b] {
                q.push(gi);
                in_q[gi] = true;
            }
        }
    }

    let mut result = 0usize;
    for (i, &v) in dw.outputs.iter().enumerate() {
        if values[v] {
            result |= 1 << i;
        }
    }
    return result;
}

#[derive(Clone, Copy)]
enum Gate {
    AND(usize, usize, usize),
    OR(usize, usize, usize),
    XOR(usize, usize, usize),
}

struct DeviceWiring {
    initial_values: Vec<(usize, bool)>,
    gates: Vec<Gate>,
    outputs: Vec<usize>,
    n: usize,
}

fn parse_input(f: &str) -> DeviceWiring {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut reading_initial_values = true;
    let mut initial_values = vec![];
    let mut gates = vec![];
    let mut outputs = vec![];

    let mut name_to_index: HashMap<String, usize> = HashMap::new();
    let mut next_index = 0;
    fn get_name_index(
        name: &String,
        name_to_index: &mut HashMap<String, usize>,
        next_index: &mut usize,
    ) -> usize {
        if let Some(i) = name_to_index.get(name) {
            return *i;
        } else {
            name_to_index.insert(name.clone(), *next_index);
            *next_index += 1;
            return *next_index - 1;
        }
    }

    for line in reader.lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            reading_initial_values = false;
        } else if reading_initial_values {
            let (left, right) = line.split_once(' ').unwrap();
            let name = left.trim_end_matches(':');
            let i = get_name_index(&name.to_string(), &mut name_to_index, &mut next_index);
            let mut value = false;
            if right == "1" {
                value = true;
            }
            initial_values.push((i, value));
        } else {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let a = get_name_index(&parts[0].to_string(), &mut name_to_index, &mut next_index);
            let b = get_name_index(&parts[2].to_string(), &mut name_to_index, &mut next_index);
            let c = get_name_index(&parts[4].to_string(), &mut name_to_index, &mut next_index);

            let op = parts[1];
            if op == "AND" {
                gates.push(Gate::AND(a, b, c));
            } else if op == "OR" {
                gates.push(Gate::OR(a, b, c));
            } else {
                gates.push(Gate::XOR(a, b, c));
            }

            let (z_index, ok) = get_z_index(parts[0]);
            if ok {
                if z_index >= outputs.len() {
                    outputs.resize(z_index + 1, 0);
                }
                outputs[z_index] = a;
            }
            let (z_index, ok) = get_z_index(parts[2]);
            if ok {
                if z_index >= outputs.len() {
                    outputs.resize(z_index + 1, 0);
                }
                outputs[z_index] = b;
            }
            let (z_index, ok) = get_z_index(parts[4]);
            if ok {
                if z_index >= outputs.len() {
                    outputs.resize(z_index + 1, 0);
                }
                outputs[z_index] = c;
            }
        }
    }

    return DeviceWiring {
        initial_values,
        gates,
        outputs,
        n: next_index,
    };
}

fn get_z_index(x: &str) -> (usize, bool) {
    let s = x.as_bytes();
    if s[0] != b'z' {
        return (0, false);
    }
    let v = s[2] - b'0' + 10 * (s[1] - b'0');
    return (v as usize, true);
}
