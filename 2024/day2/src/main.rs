use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let reports = read_input();
    part1(&reports);
    part2(&reports);
}

fn part1(reports: &Vec<Vec<i64>>) {
    let mut safe = 0;
    for r in reports.iter() {
        if is_report_safe(r) {
            safe += 1;
        }
    }
    println!("part1 solution: {safe}");
}

fn is_report_safe(r: &Vec<i64>) -> bool {
    let mut neg = 0;
    let mut pos = 0;
    for i in 0..r.len() - 1 {
        let x = r[i] - r[i + 1];
        if x == 0 || x < -3 || x > 3 {
            return false;
        }
        if x < 0 {
            neg += 1;
        } else {
            pos += 1;
        }
    }
    return neg == 0 || pos == 0;
}

fn part2(reports: &Vec<Vec<i64>>) {
    let mut saveable = 0;
    for r in reports.iter() {
        if is_report_saveable(r) {
            saveable += 1;
        }
    }
    println!("part1 solution: {saveable}");
}

fn is_report_saveable(r: &Vec<i64>) -> bool {
    let mut neg = 0;
    let mut pos = 0;
    let mut err = 0;
    for i in 0..r.len() - 1 {
        let x = r[i] - r[i + 1];
        if x == 0 || x < -3 || x > 3 {
            err += 1;
        }
        if x < 0 {
            neg += 1;
        } else {
            pos += 1;
        }
    }
    if err == 0 && (neg == 0 || pos == 0) {
        return true;
    }
    for i in 0..r.len() {
        // what would happen if i-th element is removed
        let mut n = neg;
        let mut p = pos;
        let mut e = err;
        if i > 0 {
            let x = r[i - 1] - r[i];
            if x == 0 || x < -3 || x > 3 {
                e -= 1;
            }
            if x < 0 {
                n -= 1;
            } else {
                p -= 1;
            }
        }
        if i + 1 < r.len() {
            let x = r[i] - r[i + 1];
            if x == 0 || x < -3 || x > 3 {
                e -= 1;
            }
            if x < 0 {
                n -= 1;
            } else {
                p -= 1;
            }
        }
        if i > 0 && i + 1 < r.len() {
            let x = r[i - 1] - r[i + 1];
            if x == 0 || x < -3 || x > 3 {
                e += 1;
            }
            if x < 0 {
                n += 1;
            } else {
                p += 1;
            }
        }
        if e == 0 && (n == 0 || p == 0) {
            return true;
        }
    }
    return false;
}

fn read_input() -> Vec<Vec<i64>> {
    let file = File::open("input1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Vec<i64>> = vec![];
    for line in reader.lines() {
        let s = line.unwrap();
        let mut row: Vec<i64> = vec![];
        for p in s.split_whitespace() {
            let v: i64 = p.trim().parse().unwrap();
            row.push(v);
        }
        result.push(row);
    }
    return result;
}
