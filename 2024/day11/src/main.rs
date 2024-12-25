use std::collections::{HashMap, HashSet};

fn main() {
    let input = vec![890, 0, 1, 935698, 68001, 3441397, 7221, 27];
    part1(&input);
    part2(&input);
}

fn part1(x: &Vec<i64>) {
    let mut curr: Vec<i64> = x.to_vec();
    for _ in 0..25 {
        let mut next = vec![];
        for &v in curr.iter() {
            if v == 0 {
                next.push(1);
            } else {
                let mut digit_count = 0;
                let mut z = v;
                while z > 0 {
                    digit_count += 1;
                    z = z / 10;
                }
                if digit_count % 2 == 0 {
                    let mut left = v;
                    let mut e = 1;
                    let mut right = 0;
                    for _ in 0..digit_count / 2 {
                        let d = left % 10;
                        right += e * d;
                        e *= 10;
                        left /= 10;
                    }
                    next.push(left);
                    next.push(right);
                } else {
                    next.push(v * 2024);
                }
            }
        }
        curr = next;
    }
    println!("{}", curr.len());
}

fn part2(x: &Vec<i64>) {
    // there is a lot of repetition
    // e.g. starting with [0] after 25 iterations we end up with ~20k stones but only 54 unique
    // values on them
    // the order of stones doesn't really matter in this problem so we can group stones with the
    // same value together and compute their next steps all at once
    let mut pows = vec![0; 17];
    pows[0] = 1;
    for i in 1..17 {
        pows[i] = pows[i - 1] * 10;
    }
    let mut curr: HashMap<i64, i64> = x.iter().map(|x| (*x, 1i64)).collect();
    for _ in 0..75 {
        let mut next: HashMap<i64, i64> = HashMap::new();
        for (&v, &c) in curr.iter() {
            if v == 0 {
                if let Some(z) = next.get_mut(&1) {
                    *z += c;
                } else {
                    next.insert(1, c);
                }
            } else {
                let mut digit_count = 0;
                let mut z = v;
                while z > 0 {
                    digit_count += 1;
                    z = z / 10;
                }
                if digit_count % 2 == 0 {
                    let left = v / pows[digit_count / 2];
                    let right = v % pows[digit_count / 2];
                    if let Some(z) = next.get_mut(&left) {
                        *z += c;
                    } else {
                        next.insert(left, c);
                    }
                    if let Some(z) = next.get_mut(&right) {
                        *z += c;
                    } else {
                        next.insert(right, c);
                    }
                } else {
                    if let Some(z) = next.get_mut(&(v * 2024)) {
                        *z += c;
                    } else {
                        next.insert(v * 2024, c);
                    }
                }
            }
        }
        curr = next;
    }

    let mut result = 0;
    for &c in curr.values() {
        result += c;
    }
    println!("{result}");
}
