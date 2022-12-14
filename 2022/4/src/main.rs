use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");

    // part 1
    // for each line check if one of the intervals is completely contained in the other
    let mut sum = 0;
    for line in lines.iter() {
        let (i1, i2) = parse_line(line);
        if i1.is_contained_in(&i2) || i2.is_contained_in(&i1) {
            sum += 1;
        }
    }
    println!("contained pairs: {}", sum);

    // part 2
    // for each line check if the intervals overlap
    let mut sum = 0;
    for line in lines.iter() {
        let (i1, i2) = parse_line(line);
        if i1.overlaps(&i2) {
            sum += 1;
        }
    }
    println!("overlapping pairs: {}", sum);
}

struct Interval {
    a: u32,
    b: u32,
}

impl Interval {
    fn is_contained_in(&self, i: &Interval) -> bool {
        if self.a >= i.a && self.b <= i.b {
            return true;
        }
        return false;
    }

    fn overlaps(&self, i: &Interval) -> bool {
        if self.b < i.a {
            return false;
        } else if i.b < self.a {
            return false;
        } else {
            return true;
        }
    }
}

fn parse_line(s: &String) -> (Interval, Interval) {
    let parts = s.split(',').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!();
    }
    return (parse_interval(parts.get(0).unwrap()), parse_interval(parts.get(1).unwrap()))
}

fn parse_interval(s: &str) -> Interval {
    let parts = s.split('-').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!();
    }
    return Interval {
        a: parts.get(0).unwrap().parse().unwrap(),
        b: parts.get(1).unwrap().parse().unwrap(),
    }
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