use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let (locks, keys) = parse_input("input.txt");
    part1(&locks, &keys);
}

fn part1(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) {
    let mut result = 0;
    for lock in locks {
        for key in keys {
            let mut fit = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    fit = false;
                    break;
                }
            }
            if fit {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn parse_input(f: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut locks = vec![];
    let mut keys = vec![];

    // each lock/keys consists of 7 rows and 5 columns followed by an empty line
    let mut curr: Vec<Vec<u8>> = vec![];
    // note: this is correct only because we added an empty line to the end of the input file
    for (i, line) in reader.lines().map(|x| x.unwrap().into_bytes()).enumerate() {
        if (i + 1) % 8 == 0 {
            if curr[0][0] == b'#' {
                locks.push(parse_lock(&curr));
            } else {
                keys.push(parse_key(&curr));
            }
            curr.clear();
        } else {
            curr.push(line);
        }
    }

    return (locks, keys);
}

fn parse_lock(x: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut result = vec![];
    for c in 0..5 {
        for r in 1..7 {
            if x[r][c] == b'.' {
                result.push(r - 1);
                break;
            }
        }
    }
    return result;
}

fn parse_key(x: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut result = vec![];
    for c in 0..5 {
        for r in (0..6).rev() {
            if x[r][c] == b'.' {
                result.push(5 - r);
                break;
            }
        }
    }
    return result;
}
