use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let (rules, updates) = read_input("input.txt");
    part1and2(&rules, &updates);
}

fn part1and2(rules: &Vec<(usize, usize)>, updates: &Vec<Vec<usize>>) {
    let mut sum_correct = 0;
    let mut sum_corrected = 0;
    let mut m: HashSet<(usize, usize)> = HashSet::new();
    for &(a, b) in rules.iter() {
        m.insert((a, b));
    }
    for update in updates.iter() {
        let mut correct = true;
        for i in 0..update.len() - 1 {
            let a = update[i];
            for j in i + 1..update.len() {
                let b = update[j];
                if m.contains(&(b, a)) {
                    correct = false;
                    break;
                }
            }
            if !correct {
                break;
            }
        }
        if correct {
            sum_correct += update[update.len() / 2];
        } else {
            let mut corrected = vec![];
            let mut cond = vec![0; 100];
            for i in 0..update.len() {
                let a = update[i];
                for j in i + 1..update.len() {
                    let b = update[j];
                    if m.contains(&(b, a)) {
                        cond[a] += 1;
                    } else if m.contains(&(a, b)) {
                        cond[b] += 1;
                    }
                }
                if cond[a] == 0 {
                    corrected.push(a);
                }
            }
            let mut i = 0;
            while i < corrected.len() {
                let a = corrected[i];
                for j in 0..update.len() {
                    let b = update[j];
                    if a == b {
                        continue;
                    }
                    if m.contains(&(a, b)) {
                        cond[b] -= 1;
                        if cond[b] == 0 {
                            corrected.push(b);
                        }
                    }
                }
                i += 1;
            }
            sum_corrected += corrected[corrected.len() / 2];
        }
    }
    println!("part1: {sum_correct}");
    println!("part2: {sum_corrected}");
}

fn read_input(f: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut parsing_rules = true;
    let mut rules: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];
    for line in reader.lines() {
        let s = line.unwrap();
        if parsing_rules {
            if s.is_empty() {
                parsing_rules = false;
            } else {
                let nums = s
                    .split('|')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                rules.push((nums[0], nums[1]));
            }
        } else {
            updates.push(
                s.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }
    return (rules, updates);
}
