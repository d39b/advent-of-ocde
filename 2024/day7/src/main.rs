use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let equations = read_input("example.txt");
    let equations = read_input("input.txt");
    part1(&equations);
    part2(&equations);
}

fn part1(equations: &Vec<(i64, Vec<i64>)>) {
    let mut result = 0;
    for (target, elements) in equations.iter() {
        let n = elements.len();
        if n == 1 {
            if elements[0] == *target {
                result += target;
            }
            continue;
        }
        for i in 0..(1 << (n - 1)) {
            let mut z = elements[0];
            for j in 1..n {
                if i & (1 << (j - 1)) != 0 {
                    z += elements[j];
                } else {
                    z *= elements[j];
                }
            }
            if z == *target {
                result += target;
                break;
            }
        }
    }
    println!("{result}");
}

fn part2(equations: &Vec<(i64, Vec<i64>)>) {
    let mut result = 0;
    for (target, nums) in equations.iter() {
        if rec(*target, nums, 0, 0) {
            result += target;
        }
    }
    println!("{result}");
}

fn rec(target: i64, nums: &Vec<i64>, i: usize, z: i64) -> bool {
    if i == 0 {
        return rec(target, nums, 1, nums[0]);
    } else if i == nums.len() - 1 {
        if (z + nums[i] == target) || (z * nums[i] == target) {
            return true;
        }
        let mut x = z;
        let mut v = nums[i];
        while v > 0 {
            x *= 10;
            v = v / 10;
        }
        x += nums[i];
        if x == target {
            return true;
        }
        return false;
    }
    if rec(target, nums, i + 1, z + nums[i]) {
        return true;
    }
    if rec(target, nums, i + 1, z * nums[i]) {
        return true;
    }
    let mut x = z;
    let mut v = nums[i];
    while v > 0 {
        x *= 10;
        v = v / 10;
    }
    x += nums[i];
    if rec(target, nums, i + 1, x) {
        return true;
    }
    return false;
}

fn read_input(f: &str) -> Vec<(i64, Vec<i64>)> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut result = vec![];
    for line in reader.lines() {
        let mut s = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim_matches(':').parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let target = s[0];
        s.remove(0);
        result.push((target, s));
    }
    return result;
}
