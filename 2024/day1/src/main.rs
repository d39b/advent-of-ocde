use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let (mut left, mut right) = read_input();
    part1(&mut left, &mut right);
    part2(&left, &right);
}

fn part1(left: &mut Vec<i64>, right: &mut Vec<i64>) {
    left.sort();
    right.sort();
    let n = left.len();
    let mut result = 0;
    for i in 0..n {
        let a = left[i];
        let b = right[i];
        if a > b {
            result += a - b;
        } else {
            result += b - a;
        }
    }
    println!("part1 solution: {result}");
}

fn part2(left: &Vec<i64>, right: &Vec<i64>) {
    let mut freq: HashMap<i64, i64> = HashMap::new();
    for n in right.iter() {
        if let Some(v) = freq.get_mut(n) {
            *v += 1;
        } else {
            freq.insert(*n, 1);
        }
    }
    let mut similarity_score = 0;
    for n in left.iter() {
        if let Some(v) = freq.get(n) {
            similarity_score += n * v;
        }
    }
    println!("part2 solution: {similarity_score}");
}

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let file = File::open("input1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut left = vec![];
    let mut right = vec![];
    for line in reader.lines() {
        let s = line.unwrap();
        let mut parts = s.split_whitespace();
        let l: i64 = parts.next().unwrap().trim().parse().unwrap();
        let r: i64 = parts.next().unwrap().trim().parse().unwrap();
        left.push(l);
        right.push(r);
    }
    return (left, right);
}
