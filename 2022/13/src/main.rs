use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input2.txt");

    // part 1
    let mut result = 0;
    for i in (0..lines.len()).step_by(3) {
        let p1 = Packet::new(&lines[i]);
        let p2 = Packet::new(&lines[i+1]);

        let z = p1.less(&p2);
        if z != 1 {
            result += i/3 + 1;
        }
    }
    println!("result: {}", result);

    // part 2
    let mut packets = vec![Packet::new(&"[[2]]".to_string()), Packet::new(&"[[6]]".to_string())];

    for line in lines.iter() {
        if  !line.is_empty() {
            packets.push(Packet::new(line));
        }
    }

    packets.sort_by(|a, b| {
        let z = a.less(b);
        if z == 0 {
            std::cmp::Ordering::Less
        } else if z == 1 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let mut result = 1;
    for i in 0..packets.len() {
        let packet = &packets[i];
        if packet.to_string() == "[[2]]" || packet.to_string() == "[[6]]" {
            result *= i+1;
        }
    }
    
    println!("result: {}", result);
}

enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl Packet {
    fn new(s: &String) -> Packet {
        if s.len() == 2 {
            return Packet::List(Vec::new());
        }

        // parse inner elements
        let mut children: Vec<Packet> = Vec::new();

        let mut i = 1;
        let mut mode = "none";

        let mut start = 0;
        let mut end = 0;

        let mut openBrackets = 0;
        let mut closedBrackets = 0;

        let bs = s.as_bytes();
        while i < s.len()-1 {
            let c = bs[i] as char;
            if mode == "none" {
                if c == '[' {
                    mode = "list";
                    start = i;
                    openBrackets = 1;
                    closedBrackets = 0;
                } else if c != ',' {
                    mode = "int";
                    start = i;
                    if i == s.len() - 2 {
                        children.push(Packet::Int(s[i..i+1].parse().unwrap()));
                    }
                }
            } else if mode == "list" {
                if c == '[' {
                    openBrackets += 1;
                } else if c == ']' {
                    closedBrackets += 1;
                    if openBrackets == closedBrackets {
                        end = i;
                        children.push(Packet::new(&s[start..end+1].to_string()));
                        mode = "none";
                    }
                }
            } else {
                if c == ',' {
                    children.push(Packet::Int(s[start..i].parse().unwrap()));
                    mode = "none";
                } else if i == s.len()-2 {
                    children.push(Packet::Int(s[start..i+1].parse().unwrap()));
                    mode = "none";
                }
            }
            i += 1;
        }
        return Packet::List(children);
    }

    fn less(&self, other: &Packet) -> u32 {
        match *self {
            Packet::Int(x) => match *other {
                Packet::Int(y) => {
                    if x < y {
                        0
                    } else if x > y {
                        1
                    } else {
                        2
                    }
                },
                Packet::List(ref y) => {
                    Packet::List(vec![Packet::Int(x)]).less(other)
                },
            },
            Packet::List(ref x) => match *other {
                Packet::Int(y) => {
                    self.less(&Packet::List(vec![Packet::Int(y)]))
                },
                Packet::List(ref y) => {
                    let mut s = 3;
                    let mut i = 0;
                    while i < x.len() && i < y.len() {
                        let z = x[i].less(&y[i]);
                        if z != 2 {
                            s = z;
                            break;
                        }
                        i += 1;
                    }

                    if s == 3 {
                        if x.len() == y.len() {
                            s = 2;
                        } else if x.len() < y.len() {
                            s = 0;
                        } else {
                            s = 1;
                        }
                    }

                    s
                },
            },
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Packet::Int(x) => x.to_string(),
            Packet::List(ref x) => {
                let mut s = String::new();
                for i in 0..x.len() {
                    let p = &x[i];
                    s.push_str(&p.to_string());
                    if i < x.len()-1 {
                        s.push(',');
                    }
                }
                s = format!("[{}]", s);
                s
            }
        }
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