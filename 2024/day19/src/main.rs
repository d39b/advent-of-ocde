use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let (towels, designs) = read_input("input.txt");
    part1(&towels, &designs);
    part2(&towels, &designs);
}

fn part1(towels: &Vec<Vec<u8>>, designs: &Vec<Vec<u8>>) {
    let mut possible = 0;
    for design in designs {
        let n = design.len();
        let mut x = vec![false; n];
        for t in towels {
            let k = t.len();
            if k > n {
                continue;
            }
            if matches(&design[0..k], t) {
                x[k - 1] = true;
            }
        }
        for i in 1..n {
            if x[i] {
                continue;
            }
            for t in towels {
                let k = t.len();
                if k >= i + 1 {
                    continue;
                }
                if !x[i - k] {
                    continue;
                }
                if matches(&design[i - k + 1..i + 1], t) {
                    x[i] = true;
                    break;
                }
            }
        }
        if x[n - 1] {
            possible += 1;
        }
    }

    println!("{possible}");
}

fn part2(towels: &Vec<Vec<u8>>, designs: &Vec<Vec<u8>>) {
    let mut sum_pos = 0;
    for design in designs {
        let n = design.len();
        let mut x = vec![0i64; n];
        for i in 0..n {
            for t in towels {
                let k = t.len();
                if k > i + 1 {
                    continue;
                } else if k == i + 1 {
                    if matches(&design[0..i + 1], t) {
                        x[i] += 1;
                    }
                } else {
                    if x[i - k] == 0 {
                        continue;
                    }
                    if matches(&design[i - k + 1..i + 1], t) {
                        x[i] += x[i - k];
                    }
                }
            }
        }
        sum_pos += x[n - 1];
    }

    println!("{sum_pos}");
}

fn matches(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

fn read_input(f: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let towels = lines[0]
        .split(",")
        .map(|x| x.trim().to_string().into_bytes())
        .collect();
    let mut designs = vec![];
    for i in 2..lines.len() {
        designs.push(lines[i].clone().into_bytes());
    }
    return (towels, designs);
}
