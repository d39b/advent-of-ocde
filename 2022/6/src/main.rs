use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::collections::hash_map::HashMap;

fn main() {
    let lines = read_file("input.txt");
    let line = lines.get(0).unwrap();

    println!("4 distinct: {}", find_distinct(line, 4));
    println!("14 distinct: {}", find_distinct(line, 14));
}

fn find_distinct(s: &String, n: usize) -> usize {
    let mut m: HashMap<u8, u32> = HashMap::new();
    let b = s.as_bytes();

    for i in 0..b.len() {
        let c = b[i];
        m.entry(c).and_modify(|v| *v += 1 ).or_insert(1);
        if i >= n {
            let c = b[i-n];
            m.entry(c).and_modify(|v| *v -= 1);
            if *m.get(&c).unwrap() == 0 {
                m.remove(&c);
            }
        }
        // part 1
        if m.len() == n {
            return i+1;
        }
    }
    return 0;
}

fn read_file(f: &str) -> Vec<String> {
    return BufReader::new(File::open(f).unwrap()).lines().map(|r| {
        if let Ok(s) = r {
            return s;
        } else {
            panic!();
        }
    }).collect::<Vec<String>>();
}