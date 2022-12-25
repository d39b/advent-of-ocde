use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::str;

fn main() {
    let lines = read_file("input.txt");

    let mut sum = 0;
    for line in lines.iter() {
        sum += snafuToDec(line);
    }

    println!("sum: {}", sum);
    println!("sum as snafu: {}", decToSnafu(sum));
}

fn snafuToDec(s: &String) -> i64 {
    let mut pow = 1;
    let bytes = s.as_bytes();
    let mut result = 0;
    for i in 0..bytes.len() {
        match bytes[bytes.len()-1-i] as char {
            '1' => result += pow,
            '2' => result += 2*pow, 
            '-' => result -= pow,
            '=' => result -= 2*pow,
            _ => {},
        }
        pow *= 5;
    }
    return result;
}

fn decToSnafu(x: i64) -> String {
    if x == 0 {
        return "0".to_string();
    }
    let mut result = String::new();
    let mut y = x;
    let mut carry = 0;
    while y > 0 || carry > 0 {
        let i = y % 5;
        match i + carry {
            0 => {
                result.push('0');
                carry = 0;
            }
            1 => { 
                result.push('1');
                carry = 0;
            },
            2 => {
                result.push('2');
                carry = 0;
            },
            3 => {
                result.push('=');
                carry = 1;
            },
            4 => {
                result.push('-');
                carry = 1;
            }
            _ => {
                result.push('0');
                carry = 1;
            },
        }
        y = y / 5;
    }
    return result.chars().rev().collect::<String>();
}

fn read_file(f: &str) -> Vec<String> {
    return BufReader::new(File::open(f).unwrap())
        .lines()
        .map(|r| {
            if let Ok(s) = r {
                return s;
            } else {
                panic!();
            }
        })
        .collect::<Vec<String>>();
}
