use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

fn main() {
    let robots = read_input("input.txt");
    // part1(&robots);
    part2(&robots);
    // find_cycles(&robots);
}

fn part1(robots: &Vec<Robot>) {
    let w = 101;
    let h = 103;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in robots {
        let mut x = r.x + 100 * r.vx;
        let mut y = r.y + 100 * r.vy;
        if x < 0 {
            x += 100 * w;
        }
        x = x % w;
        if y < 0 {
            y += 100 * h;
        }
        y = y % h;
        // each quadrant has 50 tiles in width and 51 tiles in height
        // so top left quadrant has coordinates 0 <= x <= 49 and 0 <= y <= 50
        // the other intervals are 51..100 and 52..102
        if x <= 49 {
            if y <= 50 {
                q1 += 1;
            } else if y >= 52 {
                q2 += 1;
            }
        } else if x >= 51 {
            if y <= 50 {
                q3 += 1;
            } else if y >= 52 {
                q4 += 1;
            }
        }
    }

    let result = q1 * q2 * q3 * q4;
    println!("{result}");
}

fn part2(robots: &Vec<Robot>) {
    let w = 101i32;
    let h = 103i32;

    for i in 0..100 {
        for r in robots {
            let step = 98 + i * 101;
            let mut x = r.x + step * r.vx;
            let mut y = r.y + step * r.vy;
            if x < 0 {
                x += step * w;
            }
            x = x % w;
            if y < 0 {
                y += step * h;
            }
            y = y % h;
            print!("{x} {y} ");
        }
        println!();
    }
}

fn find_cycles(robots: &Vec<Robot>) {
    let w = 101i32;
    let h = 103i32;

    for r in robots {
        let mut seen = vec![vec![false; h as usize]; w as usize];
        for step in 1..w * h + 5 {
            let mut x = r.x + step * r.vx;
            let mut y = r.y + step * r.vy;
            if x < 0 {
                x += step * w;
            }
            x = x % w;
            if y < 0 {
                y += step * h;
            }
            y = y % h;
            if seen[x as usize][y as usize] {
                println!("cycle after {step} steps");
                break;
            } else {
                seen[x as usize][y as usize] = true;
            }
        }
    }
}

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn read_input(f: &str) -> Vec<Robot> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut result = vec![];
    for line in reader.lines().map(|x| x.unwrap()) {
        let (left, right) = line.split_once(' ').unwrap();

        let (xs, ys) = left.split_once(',').unwrap();
        let x = xs.trim_start_matches("p=").parse::<i32>().unwrap();
        let y = ys.trim_start_matches("p=").parse::<i32>().unwrap();

        let (vxs, vys) = right.split_once(',').unwrap();
        let vx = vxs.trim_start_matches("v=").parse::<i32>().unwrap();
        let vy = vys.trim_start_matches("v=").parse::<i32>().unwrap();
        result.push(Robot { x, y, vx, vy });
    }
    return result;
}
