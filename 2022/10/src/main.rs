use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input1.txt");

    // part 1
    // rx[i] is the value of the register during cycle i+1
    let mut rx: Vec<i32> = Vec::new();

    let mut x: i32 = 1;
    for line in lines.iter() {
        let (instruction, arg) = parse_line(line);
        if instruction == 'n' {
            rx.push(x);
        } else {
            rx.push(x);
            rx.push(x);
            x += arg;
        }
    }
    rx.push(x);

    let targets = vec![20, 60, 100, 140, 180, 220];

    let mut result = 0;
    for t in targets {
        result += (t as i32) * rx[t-1];
    }

    println!("sum of signal strengths: {}", result);

    for i in 0..6 {
        let mut s = String::new();
        for j in 0..40 {
            let v = rx[i*40+j] as usize;
            if j >= v-1 && j <= v+1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        println!("{}", &s);
    }
}

// parse an input line into the movement direction and number of steps
fn parse_line(s: &String) -> (char, i32) {
    if s.starts_with('n') {
        return ('n', 0);
    }
    let parts =  s.split(' ').collect::<Vec<&str>>();
    return ('a', parts[1].parse().unwrap());
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