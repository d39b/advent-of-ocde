use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let x = read_input("example.txt");
    let x = read_input("input.txt");
    part1(&x);
    part2(&x);
}

fn part1(grid: &Vec<Vec<u8>>) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let deltas = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let xmas = "XMAS".as_bytes();

    let mut result = 0;
    for r in 0..m {
        for c in 0..n {
            for (dr, dc) in deltas.iter() {
                let er = r + 3 * dr;
                let ec = c + 3 * dc;
                if er < 0 || ec < 0 || er >= m || ec >= n {
                    continue;
                }
                let mut valid = true;
                for i in 0..4 {
                    let nr = r + i * dr;
                    let nc = c + i * dc;
                    if grid[nr as usize][nc as usize] != xmas[i as usize] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    result += 1;
                }
            }
        }
    }

    println!("{result}");
}

fn part2(grid: &Vec<Vec<u8>>) {
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let deltas = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let xmas = "XMAS".as_bytes();

    let mut result = 0;
    for r in 1..m - 1 {
        for c in 1..n - 1 {
            if grid[r as usize][c as usize] != b'A' {
                continue;
            }
            if r - 1 < 0 || r + 1 >= m || c - 1 < 0 || c + 1 >= n {
                continue;
            }
            // first diagonal
            let a1 = grid[(r - 1) as usize][(c - 1) as usize];
            let a2 = grid[(r + 1) as usize][(c + 1) as usize];
            if !((a1 == b'M' && a2 == b'S') || (a1 == b'S' && a2 == b'M')) {
                continue;
            }
            // second diagonal
            let b1 = grid[(r + 1) as usize][(c - 1) as usize];
            let b2 = grid[(r - 1) as usize][(c + 1) as usize];
            if !((b1 == b'M' && b2 == b'S') || (b1 == b'S' && b2 == b'M')) {
                continue;
            }
            result += 1;
        }
    }

    println!("{result}");
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Vec<u8>> = vec![];
    for line in reader.lines() {
        result.push(line.unwrap().into_bytes());
    }
    return result;
}
