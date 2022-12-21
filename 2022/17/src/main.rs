use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input2.txt");
    let jets = lines[0].as_bytes();

    // part 1
    // do just a straight simulation
    // for each of the seven pos keep the y of the highest rock in that column
    // we can just keep the whole board, it's not too large
    // 7*2022*4
    // implement a Rock struct that has methods to move left, right, down
    //  down return true/false depending on whether or not the rock stopped

    /*
    let mut board: Vec<Vec<bool>> = vec![vec![false; 3000*4]; 7];

    let n_rocks = 2022;
    let mut top_y = 0;

    let mut step = 0;
    for i in 0..n_rocks {
        let mut rock = Rock::new(i % 5, 2, top_y+3);
        loop {
            if jets[step % jets.len()] as char == '>' {
                rock.move_right(&board);
            } else {
                rock.move_left(&board);
            }

            step += 1;

            if !rock.move_down(&board) {
                break;
            }
        }
        for p in rock.e.iter() {
            board[p.x as usize][p.y as usize] = true;
            if p.y + 1 > top_y {
                top_y = p.y + 1;
            }
        }
    }

    println!("tower height: {}", top_y);
    */

    // part 2
    // 1_000_000_000_000 rocks
    // we can't directly simulate it, there must be some trick
    // maybe the same pattern repeats?

    let mut board: Vec<Vec<bool>> = vec![vec![false; 10000000]; 7];

    let n_rocks = 1000000;
    let mut top_y = 0;

    let mut heights: Vec<u64> = Vec::new();

    let mut step = 0;
    for i in 0..n_rocks {
        let mut rock = Rock::new(i % 5, 2, top_y+3);
        loop {
            if jets[step % jets.len()] as char == '>' {
                rock.move_right(&board);
            } else {
                rock.move_left(&board);
            }

            step += 1;

            if !rock.move_down(&board) {
                break;
            }
        }
        for p in rock.e.iter() {
            board[p.x as usize][p.y as usize] = true;
            if p.y + 1 > top_y {
                top_y = p.y + 1;
            }
        }
        heights.push(top_y);
    }

    // the following code tries to find a pattern in the heights of the tower after every rock
    // the idea is that after some number "offset" of initial rocks, the gain
    // in height in the tower repeats every "rock_count" rocks
    // i.e. there is a repeating pattern

    // for offset in 400..500 {
    //     let of = heights[offset];
    //     for rock_count in 10..100000 as u64 {
    //         // first offset rocks don't count, after that every rock_count rocks
    //         // should yield the same height
    //         let mut c = 0;
    //         let mut diff = 0;
    //         for j in 0..((n_rocks as u64)/rock_count) as u64 {
    //             let a = heights[(j*rock_count + of) as usize];
    //             let b = heights[((j+1)*rock_count + of) as usize];
    //             if diff == 0 {
    //                 diff = b-a;
    //             } else if diff == b-a {
    //                 c += 1;
    //             } else {
    //                 break;
    //             }
    //         }
    //         if c > 50 {
    //             println!("offset: {}, rockcount: {}: diff: {}", offset, rock_count, diff);
    //         }
    //     }
    // }

    // it seems to work with an offset of 400+ 
    // and repeats every 1710 rocks, growing 2620 units each time
    // so with that we should be able to calculate the height for any number of rocks

    let n_sim: u64 = 1_000_000_000_000;
    let offset = 409;
    let period = 1710;
    let height_per_period = 2620;
    let repeats = (n_sim - offset) / period;
    let remain = (n_sim - offset) % period;
    // heights[offset+remain] is for the initial rocks before the pattern loops and the remaining rocks of the last iteration
    let result = repeats * height_per_period + heights[(offset+remain) as usize] - 1;

    println!("result: {}", result);
}

fn print_board(b: &Vec<Vec<bool>>, n: usize) {
    for i in 0..n {
        let mut line = String::new();
        for j in 0..7 {
            if b[j][n-1-i] {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn print_board_with_rock(b: &Vec<Vec<bool>>, r: &Rock, n: usize) {
    for i in 0..n {
        let mut line = String::new();
        for j in 0..7 {
            if b[j][n-1-i] {
                line.push('#');
            } else {
                let mut found = false;
                for p in r.e.iter() {
                    if p.x == (j as u64) && p.y == ((n-1-i) as u64) {
                        found = true;
                        break;
                    }
                }
                if found {
                    line.push('@');
                } else {
                    line.push('.');
                }
            }
        }
        println!("{}", line);
    }
}

struct Rock {
    e: Vec<Point>,
}

impl Rock {
    fn new(i: u32, xoffset: u64, yoffset: u64) -> Rock {
        let pf = PointFactory{ xoffset: xoffset, yoffset: yoffset };
        match i {
            0 => Rock { e: vec![
                pf.new(0, 0),
                pf.new(1, 0),
                pf.new(2, 0),
                pf.new(3, 0),
            ]},
            1 => Rock { e: vec![
                pf.new(0, 1),
                pf.new(1, 0),
                pf.new(1, 1),
                pf.new(1, 2),
                pf.new(2, 1),
            ]},
            2 => Rock { e: vec![
                pf.new(0, 0),
                pf.new(1, 0),
                pf.new(2, 0),
                pf.new(2, 1),
                pf.new(2, 2),
            ]},
            3 => Rock { e: vec![
                pf.new(0, 0),
                pf.new(0, 1),
                pf.new(0, 2),
                pf.new(0, 3),
            ]},
            _ => Rock { e: vec![
                pf.new(0, 0),
                pf.new(0, 1),
                pf.new(1, 0),
                pf.new(1, 1),
            ]},
        }
    }

    fn can_move_right(&self, b: &Vec<Vec<bool>>) -> bool {
        for p in self.e.iter() {
            if !p.can_move_right(b) {
                return false;
            }
        }
        return true;
    }

    fn can_move_left(&self, b: &Vec<Vec<bool>>) -> bool {
        for p in self.e.iter() {
            if !p.can_move_left(b) {
                return false;
            }
        }
        return true;
    }

    fn can_move_down(&self, b: &Vec<Vec<bool>>) -> bool {
        for p in self.e.iter() {
            if !p.can_move_down(b) {
                return false;
            }
        }
        return true;
    }

    fn move_right(&mut self, b: &Vec<Vec<bool>>) {
        if self.can_move_right(b) {
            for p in self.e.iter_mut() {
                p.x += 1;
            }
        }
    }

    fn move_left(&mut self, b: &Vec<Vec<bool>>) {
        if self.can_move_left(b) {
            for p in self.e.iter_mut() {
                p.x -= 1;
            }
        }
    }

    fn move_down(&mut self, b: &Vec<Vec<bool>>) -> bool {
        if self.can_move_down(b) {
            for p in self.e.iter_mut() {
                p.y -= 1;
            }
            return true;
        }
        return false;
    }

    fn print(&self) {
        for p in self.e.iter() {
            println!("{}, {}", p.x, p.y);
        }
        println!()
    }
}

struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn can_move_right(&self, b: &Vec<Vec<bool>>) -> bool {
        let x = (self.x + 1) as usize;
        if x >= b.len() {
            return false;
        } else if b[x][self.y as usize] {
            return false;
        } else {
            return true;
        }
    }

    fn can_move_left(&self, b: &Vec<Vec<bool>>) -> bool {
        if self.x == 0 {
            return false;
        }
        let x = (self.x - 1) as usize;
        if b[x][self.y as usize] {
            return false;
        } else {
            return true;
        }
    }

    fn can_move_down(&self, b: &Vec<Vec<bool>>) -> bool {
        if self.y == 0 {
            return false;
        }
        let y = (self.y - 1) as usize;
        if b[self.x as usize][y] {
            return false;
        } else {
            return true;
        }
    }
}

struct PointFactory {
    xoffset: u64,
    yoffset: u64,
}

impl PointFactory {
    fn new(&self, x: u64, y: u64) -> Point {
        return Point { x: x + self.xoffset, y: y + self.yoffset };
    }
}


fn parse_line(s: &String) -> (&str, i64, Vec<&str>) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    let node = parts[1];
    let flow_rate = parse_flow_rate(parts[4]);

    let mut nbs = Vec::new();
    for i in 9..parts.len() {
        nbs.push(parts[i].trim_end_matches(','));
    }

    return (node, flow_rate, nbs);
}

fn parse_flow_rate(s: &str) -> i64 {
    let parts = s.trim_end_matches(|c| c == ';').split('=').collect::<Vec<&str>>();
    return parts[1].parse().unwrap();
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