use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::collections::hash_map::HashMap;

fn main() {
    let lines = read_file("input.txt");

    // part 1
    let mut visited: HashMap<String, bool> = HashMap::new();

    let mut head = Knot { x: 0, y: 0};
    let mut tail = Knot { x: 0, y: 0};
    visited.insert(pos_to_str(tail.x, tail.y), true);

    for line in lines.iter() {
        let (dir, count) = parse_line(line);
        for _ in 0..count {
            head.mv(dir);
            tail.follow(&head);

            // if the tail doesn't actually move this insert is useless
            // but runtime is so short who cares about this overhead
            visited.insert(pos_to_str(tail.x, tail.y), true);
        }
    }

    println!("number of visited positions: {}", visited.len());

    // part 2
    // 10 knots in the rope instead of 2

    // number of knots
    let mut visited: HashMap<String, bool> = HashMap::new();
    let n = 10;
    let target_knot = 9;
    let mut knots = vec![Knot{x:0, y: 0}; n];
    visited.insert(pos_to_str(knots[target_knot].x, knots[target_knot].y), true);

    for line in lines.iter() {
        let (dir, count) = parse_line(line);
        for _ in 0..count {
            knots[0].mv(dir);
            //println!("move: {} {} other: {} {}", &knots[target_knot].x, &knots[target_knot].y, &knots[target_knot-1].x, &knots[target_knot-1].y);
            for i in 1..n {
                let other = &knots[i-1].clone();
                knots[i].follow(&other);
            }

            // if the tail doesn't actually move this insert is useless
            // but runtime is so short who cares about this overhead
            visited.insert(pos_to_str(knots[target_knot].x, knots[target_knot].y), true);
        }
    }

    println!("number of visited positions: {}", visited.len());
    //print_grid(&visited);
}

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn mv(&mut self, dir: char) {
        match dir {
           'R' => self.x += 1,
           'L' => self.x -= 1,
           'U' => self.y -= 1,
           'D' => self.y += 1,
           _ =>  (), 
        }
    }

    fn follow(&mut self, other: &Knot) {
        let hx = other.x;
        let hy = other.y;
        let tx = self.x;
        let ty = self.y;
        if hx == tx {
            // same row
            if hy == ty + 2 {
                self.y += 1;
            } else if hy == ty - 2 {
                self.y -= 1;
            }
        } else if hy == ty {
            // same column
            if hx == tx + 2 {
                self.x += 1;
            } else if hx == tx - 2 {
                self.x -= 1;
            }
        } else {
            // they are diagonal, move if they are not touching
            if hx == tx + 2 {
                self.x += 1;
                if hy > ty {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            } else if hx == tx - 2 {
                self.x -= 1;
                if hy > ty {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            } else if hy == ty + 2 {
                self.y += 1;
                if hx > tx {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
            } else if hy == ty - 2 {
                self.y -= 1;
                if hx > tx {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
            }
        }
    }
}

fn pos_to_str(x: i32, y: i32) -> String {
    let mut result = String::from(x.to_string());
    result.push_str("|");
    result.push_str(&y.to_string());
    return result;
}

fn str_to_pos(s: &String) -> (i32, i32) {
    let parts = s.split('|').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!();
    }
    return (parts[0].parse().unwrap(), parts[1].parse().unwrap());
}

// parse an input line into the movement direction and number of steps
fn parse_line(s: &String) -> (char, u32) {
    let parts =  s.split(' ').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!();
    }
    return (parts[0].as_bytes()[0] as char, parts[1].parse().unwrap());
}

fn print_grid(pos: &HashMap<String, bool>) {
    let mut minx = 0;
    let mut maxx: i32 = 0;
    let mut miny = 0;
    let mut maxy: i32 = 0;
    for (key, value) in pos {
        let (x, y) = str_to_pos(key);
        if x < minx {
            minx = x;
        }
        if x > maxx {
            maxx = x;
        }
        if y < miny {
            miny = y;
        }
        if y > maxy {
            maxy = y;
        }
    }

    for i in miny..=maxy {
        let mut s = String::from(format!("{:02}", i.abs())); 
        for j in minx..=maxx {
            if pos.contains_key(&pos_to_str(j, i)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        println!("{}", &s);
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