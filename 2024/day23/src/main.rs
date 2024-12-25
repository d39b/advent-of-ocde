use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

fn main() {
    let edges = read_input("input.txt");
    part1and2(&edges);
}

fn part1and2(edges: &Vec<((u8, u8), (u8, u8))>) {
    let mut next_index = 0;
    let mut name_to_index: HashMap<(u8, u8), usize> = HashMap::new();

    fn get_index(
        x: &(u8, u8),
        name_to_index: &mut HashMap<(u8, u8), usize>,
        next_index: &mut usize,
    ) -> usize {
        if let Some(v) = name_to_index.get(x) {
            return *v;
        } else {
            name_to_index.insert((x.0, x.1), *next_index);
            *next_index += 1;
            return *next_index - 1;
        }
    }

    for (a, b) in edges {
        get_index(a, &mut name_to_index, &mut next_index);
        get_index(b, &mut name_to_index, &mut next_index);
    }

    let n = next_index;
    let mut adj = vec![vec![false; n]; n];
    let mut starts_with_t = vec![false; n];

    for (a, b) in edges {
        let ai = get_index(a, &mut name_to_index, &mut next_index);
        let bi = get_index(b, &mut name_to_index, &mut next_index);
        adj[ai][bi] = true;
        adj[bi][ai] = true;
        if a.0 == b't' {
            starts_with_t[ai] = true;
        }
        if b.0 == b't' {
            starts_with_t[bi] = true;
        }
    }

    let mut result = 0;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            if !adj[i][j] {
                continue;
            }
            for k in j + 1..n {
                if !(adj[i][k] && adj[j][k]) {
                    continue;
                }
                if starts_with_t[i] || starts_with_t[j] || starts_with_t[k] {
                    result += 1;
                }
            }
        }
    }

    println!("{result}");

    // part 2
    // need to find the largest complete subgraph/clique
    // how?
    // suppose there is a subgraph of size k, how can we find it
    // every node that is part of it needs degree >= k-1
    // BUT maybe this problem is a lot easier because the input has special structure
    // we should maybe first try to find the connected components of the graph

    // find connected components with union-find
    // let mut uf = vec![0; n];
    // for i in 0..n {
    //     uf[i] = i;
    // }
    // let mut degree = vec![0; n];
    // for (a, b) in edges {
    //     let ai = get_index(a, &mut name_to_index, &mut next_index);
    //     let bi = get_index(b, &mut name_to_index, &mut next_index);
    //     degree[ai] += 1;
    //     degree[bi] += 1;
    //     let pa = find(ai, &mut uf);
    //     let pb = find(bi, &mut uf);
    //     if pa != pb {
    //         union(pa, pb, &mut uf);
    //     }
    // }

    // YES! every node has degree 13 and the graph is connected
    // so max clique size could be 13
    // it cannot be 14 because otherwise that clique would not be connected to the rest of the
    // graph
    // lets find the largest clique that contains node v
    // we can iterate over all subsets of the 13 neighbors of v
    // there are 2^13 ~ 8k of these, we don't have to consider all of these, since the clique will
    // be > 3
    // we want for each node an adjacency list
    // for each subset we can precompute the set of indices that are set to 1
    // can then use these to check if every edge is there using the adjacency matrix
    // this will take roughly 13*12 checks
    // can we do a bit better?
    // yes we can probably sort by subset size and move our way from 13 down
    // for every size we will check every node and see if it is part of a clique of that size

    let mut adj_list = vec![vec![]; n];
    for (a, b) in edges {
        let ai = get_index(a, &mut name_to_index, &mut next_index);
        let bi = get_index(b, &mut name_to_index, &mut next_index);
        adj_list[ai].push(bi);
        adj_list[bi].push(ai);
    }

    let mut subsets_by_size: Vec<Vec<usize>> = vec![vec![]; 14];
    let mut subset_indices: Vec<Vec<usize>> = vec![vec![]; 1 << 13];
    for s in 0..(1 << 13) {
        let mut size = 0;
        for j in 0..13 {
            if (1 << j) & s > 0 {
                size += 1;
                subset_indices[s].push(j);
            }
        }
        subsets_by_size[size].push(s);
    }

    let mut clique = vec![];
    'outer: for size in (3..14).rev() {
        for &subset in subsets_by_size[size].iter() {
            for v in 0..n {
                let si = subset_indices.get(subset).unwrap();
                let mut valid = true;
                for i in 0..size - 1 {
                    let a = adj_list[v][si[i]];
                    for j in i + 1..size {
                        let b = adj_list[v][si[j]];
                        if !adj[a][b] {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        break;
                    }
                }
                if valid {
                    println!("found clique of size {}", size + 1);
                    clique.push(v);
                    for i in 0..size {
                        let a = adj_list[v][si[i]];
                        clique.push(a);
                    }
                    break 'outer;
                }
            }
        }
    }

    let mut names = vec![];
    for (&name, &index) in name_to_index.iter() {
        for &c in clique.iter() {
            if index == c {
                names.push(name);
            }
        }
    }

    names.sort();

    let mut password = vec![];
    for (i, &(a, b)) in names.iter().enumerate() {
        if i > 0 {
            password.push(b',');
        }
        password.push(a);
        password.push(b);
    }

    let password_string = str::from_utf8(&password).unwrap();
    println!("{password_string}");
}

// fn union(a: usize, b: usize, uf: &mut Vec<usize>) {
//     uf[a] = b;
// }
//
// fn find(x: usize, uf: &mut Vec<usize>) -> usize {
//     if uf[x] == x {
//         return x;
//     }
//     uf[x] = find(uf[x], uf);
//     return uf[x];
// }

fn read_input(f: &str) -> Vec<((u8, u8), (u8, u8))> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut result = vec![];
    for line in reader.lines().map(|x| x.unwrap().into_bytes()) {
        let a = (line[0], line[1]);
        let b = (line[3], line[4]);
        result.push((a, b));
    }
    return result;
}
