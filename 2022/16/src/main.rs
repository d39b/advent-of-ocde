use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::cmp::{min, max};

fn main() {
    let lines = read_file("input.txt");

    let n = lines.len();
    println!("{}", n);
    
    // part 1
    // use dynamic programming
    // note that only a few valves actually have a non-zero flow_rate
    // these are the only wants that we might want to open
    // there are 15 such valves in the input, so that we can represent any subset
    // of opened valves as a bitmask, and it is feasible to iterate over all 2^15 ~ 32k such sets
    // to find the solution we can compute the table
    // d[i, j, k] where
    //   - is the current minute, 1<=i<=30
    //   - j is a subset of opened valves, 0<=j<=2^15, if bit z of j is set that means that the z-th valve with a non-zero flow rate is open 
    //   - k is the current valve we are at
    //   - d[i, j, k] is the maximum pressure we can get in i minutes if we open the subset
    //      j of valves and end up at valve k
    let mut name_to_num: HashMap<&str, i64> = HashMap::new();

    let mut i = 0;
    for line in lines.iter() {
        let (name, flow_rate, nbs) = parse_line(line);
        name_to_num.insert(name, i);
        i += 1;
    }

    let mut nodes = Vec::new();
    let mut flow_i_to_i: HashMap<i64, i64> = HashMap::new();
    let mut flow_i = 0;
    for line in lines.iter() {
        let (name, flow_rate, nbs) = parse_line(line);
        let i = *name_to_num.get(name).unwrap();
        nodes.push(Node{
            i: i,
            flow_rate: flow_rate,
            flow_i: if flow_rate > 0 { 
                flow_i += 1;
                flow_i_to_i.insert(flow_i-1, i);
                flow_i - 1
            } else {
                -1 
            },
            nbs: nbs.iter().map(|x| *name_to_num.get(x).unwrap()).collect::<Vec<i64>>(),
        })
    }

    // for node in nodes.iter() {
    //     println!("{}", node.to_string());
    // }

    let n_flow = flow_i;
    let n_sets = 1<<n_flow;
    let mut flow_for_set: Vec<i64> = Vec::new();

    for i in 0..n_sets {
        let mut flow = 0;
        for j in 0..n_flow {
            if i & (1<<j) > 0 {
                flow += nodes[(*flow_i_to_i.get(&j).unwrap()) as usize].flow_rate;
            }
        }
        flow_for_set.push(flow);
    }
    
    /*
    let start_i = *name_to_num.get("AA").unwrap();
    println!("start i: {}", start_i);
    let mut curr: Vec<Vec<i64>> = vec![vec![-1; n]; n_sets];
    curr[0][start_i as usize] = 0;

    for i in 1..31 {
        let mut next: Vec<Vec<i64>> = vec![vec![-1; n]; n_sets];
        for j in 0..n_sets {
            for k in 0..n {
                if curr[j][k] > -1 {
                    // do nothing
                    next[j][k] = max(next[j][k], curr[j][k] + flow_for_set[j]);

                    // go to a neighbour of node k
                    // does it matter if we have already been there? probably not, we might have to go back
                    for nb in nodes[k].nbs.iter() {
                        next[j][*nb as usize] = max(next[j][*nb as usize], curr[j][k] + flow_for_set[j]);
                    }

                    // turn valve k, if it has flow > 0 and has not already been turned
                    let node = &nodes[k];
                    if node.has_flow() && (j & (1<<node.flow_i)) == 0 {
                        let nextj = j | (1 << node.flow_i);
                        next[nextj][k] = max(next[nextj][k], curr[j][k] + flow_for_set[j]);
                    }
                }
            }
        }
        curr = next;
    }

    let mut result = 0;
    for j in 0..n_sets {
        for k in 0..n {
            if curr[j][k] > result {
                result = curr[j][k];
            }
        }
    }

    println!("result: {}", result);
    */

    // part 2
    // now have an additional player, but we don't really need to compute to much
    // we already have all the values d[26, j, k], we don't care where we end up
    // so we can just assume we have values d[26, j] = max(d[26,j,0],...,d[26,j,n-1])
    // we want to divide all the valves into two subsets
    let start_i = *name_to_num.get("AA").unwrap();
    println!("start i: {}", start_i);
    let mut curr: Vec<Vec<i64>> = vec![vec![-1; n]; n_sets];
    curr[0][start_i as usize] = 0;

    for i in 1..27 {
        let mut next: Vec<Vec<i64>> = vec![vec![-1; n]; n_sets];
        for j in 0..n_sets {
            for k in 0..n {
                if curr[j][k] > -1 {
                    // do nothing
                    next[j][k] = max(next[j][k], curr[j][k] + flow_for_set[j]);

                    // go to a neighbour of node k
                    // does it matter if we have already been there? probably not, we might have to go back
                    for nb in nodes[k].nbs.iter() {
                        next[j][*nb as usize] = max(next[j][*nb as usize], curr[j][k] + flow_for_set[j]);
                    }

                    // turn valve k, if it has flow > 0 and has not already been turned
                    let node = &nodes[k];
                    if node.has_flow() && (j & (1<<node.flow_i)) == 0 {
                        let nextj = j | (1 << node.flow_i);
                        next[nextj][k] = max(next[nextj][k], curr[j][k] + flow_for_set[j]);
                    }
                }
            }
        }
        curr = next;
    }

    let mut fv: Vec<i64> = vec![-1; n_sets];
    for j in 0..n_sets {
        let mut jmax = -1;
        for k in 0..n {
            if curr[j][k] > jmax {
                jmax = curr[j][k];
            }
        }
        fv[j] = jmax;
    }

    let mut result = 0;
    for j in 0..n_sets {
        if fv[j] == -1 {
            continue;
        }
        // need to check any subsets of rj
        let rj = (n_sets-1) ^ j;
        for k in 0..rj+1 {
            if rj & k == k {
                let r = fv[j] + fv[k];
                if r > result {
                    result = r;
                }
            }
        }
    }

    println!("result: {}", result);
}

struct Node {
    i: i64,
    flow_rate: i64,
    flow_i: i64,
    nbs: Vec<i64>,
}

impl Node {
    fn has_flow(&self) -> bool {
        return self.flow_rate > 0;
    }

    fn to_string(&self) -> String {
        let mut nbs = String::new();

        for i in 0..self.nbs.len() {
            nbs.push_str(&self.nbs[i].to_string().as_str());
            if i != self.nbs.len() - 1 {
                nbs.push_str(", ");
            }
        }

        return format!("i: {} flow_i: {} flow_rate: {} nbs: {}", self.i, self.flow_i, self.flow_rate, nbs);
    }
}

fn parse_line(s: &String) -> (&str, i64, Vec<&str>) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    let node = parts[1];
    let flow_rate = parse_flow_rate(parts[4]);

    let mut nbs = Vec::new();
    for i in 9..parts.len() {
        nbs.push(parts[i].trim_end_matches(','));
    }

    return (node, flow_rate, nbs);
}

fn parse_flow_rate(s: &str) -> i64 {
    let parts = s.trim_end_matches(|c| c == ';').split('=').collect::<Vec<&str>>();
    return parts[1].parse().unwrap();
}

fn read_file(f: &str) -> Vec<String> {
    return BufReader::new(File::open(f).unwrap()).lines().map(|r| {
        if let Ok(s) = r {
            return s;
        } else {
            panic!();
        }
    }).collect::<Vec<String>>();
}