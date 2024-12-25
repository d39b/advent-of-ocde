use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let grid = read_input("example.txt");
    let grid = read_input("input.txt");
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<u8>>) {
    let m = grid.len();
    let n = grid[0].len();

    let mut to_index: HashMap<u8, usize> = HashMap::new();
    let mut pos: Vec<Vec<(usize, usize)>> = vec![];
    let mut next_index = 0;
    for r in 0..m {
        for c in 0..n {
            let x = grid[r][c];
            if x != b'.' {
                if let Some(&i) = to_index.get(&x) {
                    pos[i].push((r, c));
                } else {
                    to_index.insert(x, next_index);
                    next_index += 1;
                    pos.push(vec![(r, c)]);
                }
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for freq in pos.iter() {
        let k = freq.len();
        for i in 0..k - 1 {
            let (r1, c1) = freq[i];
            for j in i + 1..k {
                let (r2, c2) = freq[j];
                let dr = (r2 as i32) - (r1 as i32);
                let dc = (c2 as i32) - (c1 as i32);
                let a = (r2 as i32) + dr;
                let b = (c2 as i32) + dc;
                if a >= 0 && b >= 0 && a < (m as i32) && b < (n as i32) {
                    antinodes.insert((a, b));
                }
                let dr = (r1 as i32) - (r2 as i32);
                let dc = (c1 as i32) - (c2 as i32);
                let a = (r1 as i32) + dr;
                let b = (c1 as i32) + dc;
                if a >= 0 && b >= 0 && a < (m as i32) && b < (n as i32) {
                    antinodes.insert((a, b));
                }
            }
        }
    }

    let result = antinodes.len();
    println!("{result}");
}

fn part2(grid: &Vec<Vec<u8>>) {
    let m = grid.len();
    let n = grid[0].len();

    let mut to_index: HashMap<u8, usize> = HashMap::new();
    let mut pos: Vec<Vec<(usize, usize)>> = vec![];
    let mut next_index = 0;
    for r in 0..m {
        for c in 0..n {
            let x = grid[r][c];
            if x != b'.' {
                if let Some(&i) = to_index.get(&x) {
                    pos[i].push((r, c));
                } else {
                    to_index.insert(x, next_index);
                    next_index += 1;
                    pos.push(vec![(r, c)]);
                }
            }
        }
    }

    let mut has_antinode = vec![vec![false; n]; m];
    for freq in pos.iter() {
        let k = freq.len();
        for i in 0..k - 1 {
            let (r1, c1) = freq[i];
            has_antinode[r1][c1] = true;
            for j in i + 1..k {
                let (r2, c2) = freq[j];
                has_antinode[r2][c2] = true;
                let dr = (r2 as i32) - (r1 as i32);
                let dc = (c2 as i32) - (c1 as i32);
                let mut v = 1;
                loop {
                    let a = (r2 as i32) + v * dr;
                    let b = (c2 as i32) + v * dc;
                    if a >= 0 && b >= 0 && a < (m as i32) && b < (n as i32) {
                        has_antinode[a as usize][b as usize] = true;
                    } else {
                        break;
                    }
                    v += 1;
                }
                let mut v = 1;
                loop {
                    let a = (r2 as i32) - v * dr;
                    let b = (c2 as i32) - v * dc;
                    if a >= 0 && b >= 0 && a < (m as i32) && b < (n as i32) {
                        has_antinode[a as usize][b as usize] = true;
                    } else {
                        break;
                    }
                    v += 1;
                }
            }
        }
    }

    let mut result = 0;
    for r in 0..m {
        for c in 0..n {
            if has_antinode[r][c] {
                result += 1;
            }
        }
    }
    println!("{result}");
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let result = reader.lines().map(|x| x.unwrap().into_bytes()).collect();
    return result;
}
