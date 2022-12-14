use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");

    // part 1
    // for each line in the file, compute the priority of the char that is contained in
    // both the first and second half of the line
    // the result is the sum of all the priorities
    let mut sum = 0;
    for line in lines.iter() {
        sum += char_to_prio(find_mixed_item(&line));
    }
    
    println!("sum: {}", sum);

    // part 2
    // each group of 3 lines have exactly one char in common
    // compute the sum of priorities of for all groups 
    let mut sum2: u32 = 0;
    for i in (0..lines.len()).step_by(3) {
        sum2 += char_to_prio(find_common(lines.get(i).unwrap(), lines.get(i+1).unwrap(), lines.get(i+2).unwrap()))
    }

    println!("sum: {}", sum2);
}

// assigns a priority to the given char
// a-z: 1-26
// A-Z: 27-52
fn char_to_prio(c: u8) -> u32 {
    match c {
        n @ 65..=90 => (n-65u8+27) as u32,
        n @ 97..=122 => (n-97u8+1) as u32,
        _ => panic!() ,
    }
}

// finds the char that is contained in both the first and second half of the string
fn find_mixed_item(s: &String) -> u8 {
    let mut first_component = HashMap::new();
    // since we only have ascii letters we can operate on the bytes
    let chars = s.as_bytes();
    for i in 0..(chars.len() / 2) {
        first_component.insert(chars[i], true);
    } 
    for i in (chars.len() / 2)..chars.len() {
        if first_component.contains_key(&chars[i]) {
            return chars[i];
        }
    }
    panic!();
}

// finds the unique common char contained in all three strings
fn find_common(s1: &String, s2: &String, s3: &String) -> u8 {
    let mut m: HashMap<u8, u8> = HashMap::new();

    let s = [s1, s2, s3];
    for i in 0..3 {
        for c in s[i].as_bytes() {
            m.entry(*c).and_modify(|mask| *mask |= 1u8 << i).or_insert(1u8 << i);
        }
    }

    for (c, v) in m {
        if v == 7 {
            return c;
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