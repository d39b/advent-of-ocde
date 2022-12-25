use core::panic;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");

    // part 1
    // input grid has size 25*120 = 2400+600 = 3000
    // there are alot of paths, especially if we consider waiting at a position as well
    // we probably can't just try and brute-force traverse the search space
    // especially since it might also make sense sometimes to move left and up (we need to go from top left to bottom right)
    // however since the blizzards move through 1 row/column and then repeat
    // there are only so many patterns that the thing goes through
    // how many steps does it take for the pattern to start fresh?
    // every row repeats every 120 steps
    // every column every 25 steps
    // so both together every least_common_multiple(25,120) ?
    // which is: 25 = 5*5, 120=5*24=5*2*2*2*3
    // lcm = 5*5*2*2*2*3 = 120*5 = 600
    // so if we view this as a graph where each node is a (x, y, i) tuple, where (x,y) is a position and i in [0,600) is the current iteration
    // then we can perform normal dijkstra
    // so we first compute this graph
    // as a[x][y][i] = true if that cell is not occupied at that time
    //  and we have neighbours for i+1, and x,y either the same or adjacent
    //  note that for a[x][y][599] we can wrap around back to 0

    let (blizzards, n, m) = parse_state(&lines);
    let k = lcm(n,m);

    println!("n: {} m: {} k: {}", n, m, k);

    let mut g = vec![vec![vec![false; k]; m]; n];
    for b in blizzards.iter() {
        match b.d {
            Dir::Up => {
                for z in 0..k {
                    // since k is a multiple n and m this works
                    g[(b.y + k - z) % n][b.x][z] = true;
                }
            },
            Dir::Down => {
                for z in 0..k {
                    g[(b.y + z) % n][b.x][z] = true;
                }
            },
            Dir::Left => {
                for z in 0..k {
                    g[b.y][((b.x + k - z) % m)][z] = true;
                }
            },
            Dir::Right => {
                for z in 0..k {
                    g[b.y][((b.x + z) % m)][z] = true;
                }
            },
        }
    }

    let (first, end_i_first)= find_shortest_path(&g, n, m, k, false, 0);
    let (second, end_i_second) = find_shortest_path(&g, n, m, k, true, end_i_first);
    let (third, _ )= find_shortest_path(&g, n, m, k, false, end_i_second);

    println!("result: {}", first + second + third);
}

fn find_shortest_path(g: &Vec<Vec<Vec<bool>>>, n: usize, m: usize, k: usize, rev: bool, start_i: usize) -> (usize, usize) {
    // start position is basically -1, 0, 0
    // we encode -1 as 1<<30
    // end position is basically n-1, m-1
    let mut q = VecDeque::new();
    let mut visited = vec![vec![vec![false; k]; m]; n];
    let mut inQueue = vec![vec![vec![false; k]; m]; n];
    if rev {
        q.push_back(Node {y: n, x: m-1, i: start_i, dist: 0});
    } else {
        q.push_back(Node {y: 1<<30, x: 0, i: start_i, dist: 0});
    }

    let result = 0;
    loop {
        let curr = q.pop_front().unwrap();
        if curr.y != 1<<30 && curr.y != n {
            visited[curr.y][curr.x][curr.i] = true;
        }
        let i = (curr.i + 1) % k;
        for nb in curr.nbs(n,m,k,g) {
            if rev {
                if nb.y == 1<<30 && nb.x == 0 {
                    return (nb.dist, i);
                }
            } else {
                if nb.y == n && nb.x == m-1 {
                    return (nb.dist, i);
                }
            }
            if nb.y == 1 << 30 || nb.y == n {
                q.push_back(nb);
            } else if !visited[nb.y][nb.x][nb.i] && !inQueue[nb.y][nb.x][nb.i] {
                inQueue[nb.y][nb.x][nb.i] = true;
                q.push_back(nb);
            }
        }
    }
}

struct Node {
    y: usize,
    x: usize,
    i: usize,
    dist: usize,
}

impl Node {
    fn nbs(&self, n: usize, m: usize, k: usize, g: &Vec<Vec<Vec<bool>>>) -> Vec<Node> {
        let i = (self.i + 1) % k;
        let dist = self.dist + 1;
        if self.y == 1<<30 {
            if !g[0][0][i] {
                return vec![Node{i: i, y: self.y, x: self.x, dist: dist}, Node{y: 0, x: 0, i: i, dist: dist}];
            } else {
                return vec![Node{i: i, y: self.y, x: self.x, dist: dist}];
            }
        }
        if self.y == n {
            if !g[n-1][m-1][i] {
                return vec![Node{i: i, y: self.y, x: self.x, dist: dist}, Node{y: n-1, x: m-1, i: i, dist: dist}];
            } else {
                return vec![Node{i: i, y: self.y, x: self.x, dist: dist}];
            }
        }
        let mut result = Vec::new();
        if !g[self.y][self.x][i] {
            result.push(Node{y: self.y, x: self.x, i: i, dist: dist});
        }
        if self.y > 0 && !g[self.y-1][self.x][i] {
            result.push(Node{y: self.y-1, x: self.x, i: i, dist: dist});
        } 
        if self.y < n-1 && !g[self.y+1][self.x][i] {
            result.push(Node{y: self.y+1, x: self.x, i: i, dist: dist});
        } 
        if self.x > 0 && !g[self.y][self.x-1][i] {
            result.push(Node{y: self.y, x: self.x-1, i: i, dist: dist});
        } 
        if self.x < m-1 && !g[self.y][self.x+1][i] {
            result.push(Node{y: self.y, x: self.x+1, i: i, dist: dist});
        } 
        if self.x == m-1 && self.y == n-1 {
            result.push(Node{y: n, x: m-1, i: i, dist: dist});
        }
        if self.x == 0 && self.y == 0 {
            result.push(Node{y: 1<<30, x: 0, i: i, dist: dist});
        }
        return result;
    }
}

fn lcm(a: usize, b: usize) -> usize {
    return a*b / gcd(a, b);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

#[derive(Clone,Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Blizzard {
    y: usize,
    x: usize,
    d: Dir,
}

fn parse_state(lines: &Vec<String>) -> (Vec<Blizzard>, usize, usize) {
    let n = lines.len() - 2;
    let m = lines[0].len() - 2;

    let mut blizzards = Vec::new();

    for i in 0..n {
        let line = lines[i+1].as_bytes();
        for j in 0..m {
            let c = line[j+1] as char;
            let b = Blizzard {
                y: i,
                x: j,
                d: Dir::Up,
            };
            if c == '>' {
                blizzards.push(Blizzard {d: Dir::Right, ..b});
            } else if c == '<' {
                blizzards.push(Blizzard {d: Dir::Left, ..b});
            } else if c == '^' {
                blizzards.push(Blizzard {d: Dir::Up, ..b});
            } else if c == 'v' {
                blizzards.push(Blizzard {d: Dir::Down, ..b});
            }
        }
    }

    return (blizzards, n, m);
}

fn read_file(f: &str) -> Vec<String> {
    return BufReader::new(File::open(f).unwrap())
        .lines()
        .map(|r| {
            if let Ok(s) = r {
                return s;
            } else {
                panic!();
            }
        })
        .collect::<Vec<String>>();
}
