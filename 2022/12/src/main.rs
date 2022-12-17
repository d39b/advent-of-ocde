use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::collections::VecDeque;

fn main() {
    let lines = read_file("input2.txt");

    let n = lines.len();
    let m = lines[0].len();
    let n_nodes = n * m;

    let ix = Indexer{ n: n, m: m};

    let mut start: usize = 0; 
    let mut end: usize = 0;

    let mut height = vec![0u8; n_nodes];

    for i in 0..n {
        for j in 0..m {
            let index = ix.zip(i, j);
            match lines[i].as_bytes()[j] as char {
                'S' => {
                    start = index;
                    height[index] = 0;
                },
                'E' => {
                    end = index;
                    height[index] = 25;
                },
                c => height[index] = (c as u8) - ('a' as u8),
            }
        }
    }

    // part 1, find shortest path from start to end
    let mut visited = vec![false; n_nodes];
    let mut inq = vec![false; n_nodes];
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![1<<30; n_nodes];
    dist[start] = 0;

    q.push_back(start);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        visited[curr] = true;

        for nb in ix.neighbours(curr) {
            if !visited[nb] && !inq[nb] {
                if height[nb] <= height[curr] + 1 {
                    dist[nb] = dist[curr] + 1;
                    q.push_back(nb);
                    inq[nb] = true;
                    if nb == end {
                        break;
                    }
                }
            }
        }
    }

    println!("dist: {}", dist[end]);

    // part 1, find shortest path from any square at elevation 'a' to end
    // we can turn this around, find shortest distances from end to all nodes
    // then among those with elevation a, choose the one with lowest distance
    let mut visited = vec![false; n_nodes];
    let mut inq = vec![false; n_nodes];
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut dist = vec![1<<30; n_nodes];
    dist[end] = 0;

    q.push_back(end);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        visited[curr] = true;

        for nb in ix.neighbours(curr) {
            if !visited[nb] && !inq[nb] {
                // now we move backward 
                // we check if we could have moved from the neighbour to the current one
                if height[curr] <= height[nb] + 1 {
                    dist[nb] = dist[curr] + 1;
                    q.push_back(nb);
                    inq[nb] = true;
                }
            }
        }
    }

    let mut result = 1 << 30;
    for i in 0..n_nodes {
        if height[i] == 0 {
            if dist[i] < result {
                result = dist[i];
            }
        }
    }

    println!("dist: {}", result);
}

struct Indexer {
    n: usize,
    m: usize,
}

impl Indexer {
    fn zip(&self, i: usize, j: usize) -> usize {
        return i*self.m + j;
    }

    fn uzip(&self, index: usize) -> (usize, usize) {
        return (index / self.m, index % self.m)
    }

    fn neighbours_uzip(&self, i: usize, j: usize) -> Vec<usize> {
        let mut result = Vec::new();
        if i > 0 {
            result.push(self.zip(i-1, j));
        }
        if i < self.n - 1 {
            result.push(self.zip(i+1, j));
        }
        if j > 0 {
            result.push(self.zip(i, j-1))
        }
        if j < self.m - 1 {
            result.push(self.zip(i, j+1))
        }
        return result;
    }

    fn neighbours(&self, index: usize) -> Vec<usize> {
        let (i, j) = self.uzip(index);
        return self.neighbours_uzip(i, j);
    }
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