use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let machines = read_input("example.txt");
    let machines = read_input("input.txt");
    part1(&machines);
    part2(&machines);
}

fn part1(machines: &Vec<Machine>) {
    let mut result = 0;
    for m in machines.iter() {
        let mut min_cost = -1;
        for a in 0..101 {
            let px = m.px - a * m.ax;
            let py = m.py - a * m.ay;
            if px < 0 || py < 0 {
                continue;
            }
            if px % m.bx == 0 {
                let b = px / m.bx;
                if py == m.by * b {
                    let cost = a * 3 + b;
                    if min_cost == -1 || cost < min_cost {
                        min_cost = cost;
                    }
                }
            }
        }
        if min_cost != -1 {
            result += min_cost;
        }
    }
    println!("{result}");
}

fn part2(machines: &Vec<Machine>) {
    let prize_delta = 10000000000000i64;
    let mut result = 0;
    for m in machines.iter() {
        // solve the linear system of equations (ax * a + bx * b = px, ay * a + by * b = py)
        // can add a multiple of first equation to second to get an equation of the form x*b = c
        // solve for b, then insert b into first equation to obtain a
        let px = m.px + prize_delta;
        let py = m.py + prize_delta;
        let pxf = px as f64;
        let pyf = py as f64;
        let ax = m.ax as f64;
        let ay = m.ay as f64;
        let bx = m.bx as f64;
        let by = m.by as f64;

        let b = (pyf - (pxf * ay) / ax) / (by - (bx * ay) / ax);
        let a = (pxf - bx * b) / ax;
        let ai = a.round() as i64;
        let bi = b.round() as i64;
        if ai < 0 || bi < 0 {
            continue;
        }
        if ai * m.ax + bi * m.bx != px {
            continue;
        }
        if ai * m.ay + bi * m.by != py {
            continue;
        }
        result += ai * 3 + bi;
    }
    println!("{result}");
}

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn read_input(f: &str) -> Vec<Machine> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut result = vec![];
    let mut i = 0;
    while i + 3 < lines.len() {
        let (ax, ay) = parse_button_line(&lines[i]);
        let (bx, by) = parse_button_line(&lines[i + 1]);
        let (px, py) = parse_prize_line(&lines[i + 2]);
        result.push(Machine {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        });
        i += 4;
    }
    return result;
}

fn parse_button_line(s: &String) -> (i64, i64) {
    let parts = s.split_whitespace().collect::<Vec<&str>>();
    let x = parts[2]
        .trim_end_matches(',')
        .trim_start_matches("X+")
        .parse()
        .unwrap();
    let y = parts[3].trim_start_matches("Y+").parse().unwrap();
    return (x, y);
}

fn parse_prize_line(s: &String) -> (i64, i64) {
    let parts = s.split_whitespace().collect::<Vec<&str>>();
    let x = parts[1]
        .trim_end_matches(',')
        .trim_start_matches("X=")
        .parse()
        .unwrap();
    let y = parts[2].trim_start_matches("Y=").parse().unwrap();
    return (x, y);
}
