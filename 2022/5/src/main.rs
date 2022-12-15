use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");

    // number of stacks
    let n = (lines.get(0).unwrap().len()+1) / 4;

    // part 1
    let mut vecs = std::iter::repeat::<Vec<u8>>(vec![]).take(n).collect::<Vec<Vec<u8>>>();
   
    let mut reading_stacks = true;
    for line in lines.iter() {
        if reading_stacks {
            let b = line.as_bytes();
            if b[1] == '1' as u8 {
                reading_stacks = false;
                // reverse all stacks
                for vec in vecs.iter_mut() {
                    vec.reverse();
                }
            }
            for i in 0..n {
                // this is only a create if it is > 64, i.e. uppercase ascii
                let cr = b[1+i*4];
                if cr > 64 {
                    vecs[i].push(b[1+i*4]);
                }
            }
        } else if !line.is_empty() {
            let (count, from, to) = parse_line(line);
            for _ in 0..count {
                let p = vecs[from].pop().unwrap();
                vecs[to].push(p);
            }
        }
    }


    let mut result = String::new();
    for vec in vecs {
        result.push(*(vec.last().unwrap()) as char);
    }
    println!("result: {}", result);

    // part 2
    let mut vecs = std::iter::repeat::<Vec<u8>>(vec![]).take(n).collect::<Vec<Vec<u8>>>();
   
    let mut reading_stacks = true;
    for line in lines.iter() {
        if reading_stacks {
            let b = line.as_bytes();
            if b[1] == '1' as u8 {
                reading_stacks = false;
                // reverse all stacks
                for vec in vecs.iter_mut() {
                    vec.reverse();
                }
            }
            for i in 0..n {
                // this is only a create if it is > 64, i.e. uppercase ascii
                let cr = b[1+i*4];
                if cr > 64 {
                    vecs[i].push(b[1+i*4]);
                }
            }
        } else if !line.is_empty() {
            let (count, from, to) = parse_line(line);
            for i in 0..count {
                let vecFrom = &vecs[from];
                let j = vecFrom.len() - count + i;
                let c = vecFrom[j];
                vecs[to].push(c);
            }
            let vecFrom = &mut vecs[from];
            vecFrom.truncate(vecFrom.len() - count);
        }
    }


    let mut result = String::new();
    for vec in vecs {
        result.push(*(vec.last().unwrap()) as char);
    }
    println!("result: {}", result);
}

fn parse_line(s: &String) -> (usize, usize, usize) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    if parts.len() != 6 {
        panic!();
    }
    return (
        parse_string(&parts, 1),
        parse_string(&parts, 3) - 1,
        parse_string(&parts, 5) - 1,
    );
}

fn parse_string(parts: &Vec<&str>, i: usize) -> usize {
    return parts.get(i).unwrap().parse().unwrap();
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