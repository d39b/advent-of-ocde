use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec;

fn main() {
    let map = read_input("input.txt");
    // let map = read_input("example.txt");
    part1(&map);
    part2(&map);
}

// we can simply start a BFS or DFS from each zero height cell and count the number of height nine
// cells reachable
fn part1(map: &Vec<Vec<u8>>) {
    // input is square
    let n = map.len();

    let mut result = 0;
    for r in 0..n {
        for c in 0..n {
            if map[r][c] == b'0' {
                let z = bfs(map, r, c);
                result += z;
            }
        }
    }
    println!("{result}");
}

fn bfs(map: &Vec<Vec<u8>>, sr: usize, sc: usize) -> i32 {
    let n = map.len();
    let mut q = vec![];
    let mut qi = 0;
    let mut visited = vec![vec![false; n]; n];
    let mut result = 0;

    q.push((sr, sc));

    while qi < q.len() {
        let (r, c) = q[qi];
        qi += 1;
        if visited[r][c] {
            continue;
        }
        visited[r][c] = true;
        let h = map[r][c];
        if h == b'9' {
            result += 1;
            continue;
        }
        let mut nbs = [(0, 0); 4];
        let mut nb_count = 0;
        if r > 0 {
            nbs[nb_count] = (r - 1, c);
            nb_count += 1;
        }
        if c > 0 {
            nbs[nb_count] = (r, c - 1);
            nb_count += 1;
        }
        if r < n - 1 {
            nbs[nb_count] = (r + 1, c);
            nb_count += 1;
        }
        if c < n - 1 {
            nbs[nb_count] = (r, c + 1);
            nb_count += 1;
        }
        for i in 0..nb_count {
            let (nr, nc) = nbs[i];
            if map[nr][nc] == h + 1 && !visited[nr][nc] {
                q.push((nr, nc));
            }
        }
    }

    return result;
}

// we can compute recursively d(r, c) = the number of trailheads that start at a cell (r, c)
// d(r, c) = 1 if the height of (r, c) is 9
// d(r, c) = sum of d(nr, nc) over all neighbors of (r, c) where height of (nr, nc) is one higher than (r, c)
// we could again do BFS or DFS or simply iterate over the cells by height in decreasing order
fn part2(map: &Vec<Vec<u8>>) {
    // input is square
    let n = map.len();

    let mut d = vec![vec![0; n]; n];

    let mut cells = vec![vec![]; 9];

    for r in 0..n {
        for c in 0..n {
            if map[r][c] == b'9' {
                d[r][c] = 1;
            } else {
                cells[(map[r][c] - b'0') as usize].push((r, c));
            }
        }
    }

    for h in (0..9).rev() {
        for &(r, c) in cells[h].iter() {
            let mut nbs = [(0, 0); 4];
            let mut nb_count = 0;
            if r > 0 {
                nbs[nb_count] = (r - 1, c);
                nb_count += 1;
            }
            if c > 0 {
                nbs[nb_count] = (r, c - 1);
                nb_count += 1;
            }
            if r < n - 1 {
                nbs[nb_count] = (r + 1, c);
                nb_count += 1;
            }
            if c < n - 1 {
                nbs[nb_count] = (r, c + 1);
                nb_count += 1;
            }
            let th = (h as u8) + b'0' + 1;
            for i in 0..nb_count {
                let (nr, nc) = nbs[i];
                if map[nr][nc] == th {
                    d[r][c] += d[nr][nc];
                }
            }
        }
    }

    let mut result = 0;
    for &(r, c) in cells[0].iter() {
        result += d[r][c];
    }
    println!("{result}");
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let result = reader.lines().map(|x| x.unwrap().into_bytes()).collect();
    return result;
}
