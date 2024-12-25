use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let secrets = read_input("input.txt");
    // let secrets = vec![1, 10, 100, 2024];
    // let secrets = vec![1, 2, 3, 2024];
    part1(&secrets);
    part2(&secrets);
}

const M: i64 = 16777216;

fn part1(secrets: &Vec<i64>) {
    let mut result = 0;
    for &s in secrets {
        let mut curr = s;
        for _ in 0..2000 {
            curr = (curr ^ (curr << 6)) % M;
            curr = (curr ^ (curr >> 5)) % M;
            curr = (curr ^ (curr << 11)) % M;
        }
        result += curr;
    }
    println!("{result}");
}

fn part2(secrets: &Vec<i64>) {
    let mut v = vec![0; 2001];
    let mut diff = vec![0; 2000];
    let n = 19 * 19 * 19 * 19;
    let mut g = vec![0; n];
    for &s in secrets {
        let mut curr = s;
        v[0] = s % 10;
        for i in 0..2000 {
            curr = (curr ^ (curr << 6)) % M;
            curr = (curr ^ (curr >> 5)) % M;
            curr = (curr ^ (curr << 11)) % M;
            v[i + 1] = curr % 10;
            diff[i] = v[i + 1] - v[i] + 9;
        }

        let mut l = vec![false; n];
        for i in 3..2000 {
            let price = v[i + 1];
            let z = diff[i] + diff[i - 1] * 19 + diff[i - 2] * 19 * 19 + diff[i - 3] * 19 * 19 * 19;
            if !l[z as usize] {
                l[z as usize] = true;
                g[z as usize] += price;
            }
        }
    }

    let mut max = 0;
    for &price in g.iter() {
        if price > max {
            max = price;
        }
    }

    println!("{max}");
}

fn read_input(f: &str) -> Vec<i64> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}
