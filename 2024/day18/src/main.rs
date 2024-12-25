use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let bytes = read_input("input.txt");
    part1(&bytes);
    part2(&bytes);
}

fn part1(bytes: &Vec<(usize, usize)>) {
    let n = 71;

    let mut grid = vec![vec![false; n]; n];

    for i in 0..1024 {
        let (c, r) = bytes[i];
        grid[r][c] = true;
    }

    let mut d = vec![vec![-1; n]; n];
    d[0][0] = 0;
    let mut q = vec![(0, 0, 0)];
    let mut qi = 0;

    while qi < q.len() {
        let (r, c, dist) = q[qi];
        qi += 1;
        if d[r][c] != dist {
            continue;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = (r as i64) + dr;
            let nc = (c as i64) + dc;
            if nr < 0 || nc < 0 || nr >= (n as i64) || nc >= (n as i64) {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if grid[nr][nc] {
                continue;
            }
            let z = dist + 1;
            if d[nr][nc] == -1 || z < d[nr][nc] {
                d[nr][nc] = z;
                q.push((nr, nc, z));
            }
        }
    }

    println!("{}", d[n - 1][n - 1]);
}

// use binary search to find the byte that blocks the path
fn part2(bytes: &Vec<(usize, usize)>) {
    let n = 71;

    let mut low = 1024;
    let mut high = bytes.len() - 1;
    let mut result = 0;
    while low <= high {
        let mid = (low + high) / 2;
        let mut grid = vec![vec![false; n]; n];
        for i in 0..mid + 1 {
            let (c, r) = bytes[i];
            grid[r][c] = true;
        }
        if is_reachable(&grid) {
            low = mid + 1;
        } else {
            result = mid;
            high = mid - 1;
        }
    }
    println!("{} {}", bytes[result].0, bytes[result].1);
}

fn is_reachable(grid: &Vec<Vec<bool>>) -> bool {
    let n = grid.len();
    let mut reachable = vec![vec![false; n]; n];
    reachable[0][0] = true;
    let mut q = vec![(0, 0)];
    let mut qi = 0;

    while qi < q.len() {
        let (r, c) = q[qi];
        qi += 1;

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = (r as i64) + dr;
            let nc = (c as i64) + dc;
            if nr < 0 || nc < 0 || nr >= (n as i64) || nc >= (n as i64) {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if grid[nr][nc] {
                continue;
            }
            if !reachable[nr][nc] {
                reachable[nr][nc] = true;
                q.push((nr, nc));
            }
        }
    }
    return reachable[n - 1][n - 1];
}

fn read_input(f: &str) -> Vec<(usize, usize)> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let result = reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (l, r) = x.split_once(',').unwrap();
            let a = l.parse::<usize>().unwrap();
            let b = r.parse::<usize>().unwrap();
            (a, b)
        })
        .collect::<Vec<(usize, usize)>>();
    return result;
}
