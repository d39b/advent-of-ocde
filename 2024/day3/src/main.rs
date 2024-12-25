use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let x = read_input();
    // part1(&x);
    // let x = read_input2("example1.txt");
    let x = read_input2("input1.txt");
    part2(&x);
}

fn part1(x: &Vec<(i64, i64)>) {
    let mut result = 0;
    for (a, b) in x.iter() {
        result += a * b;
    }
    println!("{result}");
}

fn part2(x: &Vec<Instruction>) {
    let mut result = 0;
    let mut enabled = true;
    for i in x.iter() {
        match i {
            Instruction::Mul(x, y) => {
                if enabled {
                    result += x * y;
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
        }
    }
    println!("{result}");
}

fn read_input() -> Vec<(i64, i64)> {
    let file = File::open("input1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<(i64, i64)> = vec![];
    for line in reader.lines() {
        let r = parse(line.unwrap());
        result.extend(r);
    }
    return result;
}

#[derive(Debug)]
enum ParseState {
    Start,
    Num1,
    Num2,
}

fn parse(s: String) -> Vec<(i64, i64)> {
    let mut result = vec![];
    let s = s.as_bytes();
    let n = s.len();
    let mut i = 0;
    let mut num1_start = 0;
    let mut num1_end = 0;
    let mut num2_start = 0;
    let mut num2_end = 0;
    let mut state = ParseState::Start;
    while i < n {
        match state {
            ParseState::Start => {
                if i + 3 < n {
                    if s[i..i + 4].starts_with("mul(".as_bytes()) {
                        state = ParseState::Num1;
                        num1_start = i + 4;
                        num1_end = i + 4;
                        i += 4;
                    } else {
                        i += 1;
                    }
                } else {
                    break;
                }
            }
            ParseState::Num1 => {
                if s[i].is_ascii_digit() {
                    num1_end += 1;
                    i += 1;
                } else if s[i] == b',' {
                    if num1_start >= num1_end {
                        state = ParseState::Start;
                    } else {
                        state = ParseState::Num2;
                        num2_start = i + 1;
                        num2_end = i + 1;
                    }
                    i += 1;
                } else {
                    state = ParseState::Start;
                }
            }
            ParseState::Num2 => {
                if s[i].is_ascii_digit() {
                    num2_end += 1;
                    i += 1;
                } else if s[i] == b')' {
                    if num2_start >= num2_end {
                        state = ParseState::Start;
                    } else {
                        let num1 = parse_num(&s[num1_start..num1_end]);
                        let num2 = parse_num(&s[num2_start..num2_end]);
                        result.push((num1, num2));
                        state = ParseState::Start;
                    }
                    i += 1;
                } else {
                    state = ParseState::Start;
                }
            }
        }
    }
    return result;
}

fn parse_num(s: &[u8]) -> i64 {
    let mut result = 0;
    for x in s.iter() {
        result *= 10;
        result += (x - b'0') as i64;
    }
    return result;
}

enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

fn read_input2(f: &str) -> Vec<Instruction> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Instruction> = vec![];
    for line in reader.lines() {
        let r = parse2(line.unwrap());
        result.extend(r);
    }
    return result;
}

// for part 2
fn parse2(s: String) -> Vec<Instruction> {
    let mut result = vec![];
    let s = s.as_bytes();
    let n = s.len();
    let mut i = 0;
    let mut num1_start = 0;
    let mut num1_end = 0;
    let mut num2_start = 0;
    let mut num2_end = 0;
    let mut state = ParseState::Start;
    while i < n {
        match state {
            ParseState::Start => {
                if i + 3 < n && s[i..i + 4].starts_with("do()".as_bytes()) {
                    result.push(Instruction::Do);
                    i += 4;
                } else if i + 6 < n && s[i..i + 7].starts_with("don't()".as_bytes()) {
                    result.push(Instruction::Dont);
                    i += 7;
                } else if i + 3 < n && s[i..i + 4].starts_with("mul(".as_bytes()) {
                    state = ParseState::Num1;
                    num1_start = i + 4;
                    num1_end = i + 4;
                    i += 4;
                } else if i + 3 < n {
                    i += 1;
                } else {
                    break;
                }
            }
            ParseState::Num1 => {
                if s[i].is_ascii_digit() {
                    num1_end += 1;
                    i += 1;
                } else if s[i] == b',' {
                    if num1_start >= num1_end {
                        state = ParseState::Start;
                    } else {
                        state = ParseState::Num2;
                        num2_start = i + 1;
                        num2_end = i + 1;
                    }
                    i += 1;
                } else {
                    state = ParseState::Start;
                }
            }
            ParseState::Num2 => {
                if s[i].is_ascii_digit() {
                    num2_end += 1;
                    i += 1;
                } else if s[i] == b')' {
                    if num2_start >= num2_end {
                        state = ParseState::Start;
                    } else {
                        let num1 = parse_num(&s[num1_start..num1_end]);
                        let num2 = parse_num(&s[num2_start..num2_end]);
                        result.push(Instruction::Mul(num1, num2));
                        state = ParseState::Start;
                    }
                    i += 1;
                } else {
                    state = ParseState::Start;
                }
            }
        }
    }
    return result;
}
