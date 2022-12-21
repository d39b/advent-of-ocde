use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::cmp::{min, max};

fn main() {
    let lines = read_file("input.txt");

    // the inputs are not too large, we can just brute force simulate every step 
    // of every unit of sand

    let mut points: Vec<Vec<Point>> = Vec::new();

    for line in lines.iter() {
        let p = parse_line(line);
        points.push(p);
    }

    let mut maxx = 0;
    let mut minx = 500;
    let mut maxy = 0;
    for pp in points.iter() {
        for p in pp.iter() {
            if p.x > maxx {
                maxx = p.x;
            }
            if p.x < minx {
                minx = p.x;
            }
            if p.y > maxy {
                maxy = p.y
            }
        }
    }

    // part 1
    let nx = maxx - minx + 10;
    let ny = maxy + 10;

    let mut grid = vec![vec![false; ny as usize]; (nx+20) as usize];

    for pp in points.iter() {
        for i in 0..pp.len()-1 {
            let p1 = &pp[i];
            let p2 = &pp[i+1];

            if p1.x == p2.x {
                let low = min(p1.y, p2.y);
                let high = max(p1.y, p2.y);
                let x = p1.x - minx + 10;
                for j in low..=high {
                    grid[x as usize][j as usize] = true;
                }
            } else {
                let low = min(p1.x, p2.x);
                let high = max(p1.x, p2.x);
                for j in low..=high {
                    grid[(j - minx + 10) as usize][p1.y as usize] = true;
                }
            }
        }
    }

    let mut currx = (500 - minx + 10) as usize;
    let mut curry: usize = 1;

    let mut result = 0;
    loop {
        let mut stopped = false;
        while curry <= maxy as usize {
            if !grid[currx][curry+1] {
                curry += 1;
            } else if !grid[currx-1][curry+1] {
                currx -= 1;
                curry += 1;
            } else if !grid[currx+1][curry+1] {
                currx += 1;
                curry += 1;
            } else {
                grid[currx][curry] = true;
                stopped = true;
                break;
            }
        }
        if !stopped {
            break;
        } else {
            result += 1;
            currx = (500 - minx + 10) as usize;
            curry = 1;
        }
    }

    println!("result: {}", result);

    // part 2
    let nx = maxx + 500;
    let ny = maxy + 4;

    let mut grid = vec![vec![false; ny as usize]; (nx + 500) as usize];
    
    for i in 0..nx+500 {
        grid[i as usize][(maxy+2) as usize] = true;
    }

    for pp in points.iter() {
        for i in 0..pp.len()-1 {
            let p1 = &pp[i];
            let p2 = &pp[i+1];

            if p1.x == p2.x {
                let low = min(p1.y, p2.y);
                let high = max(p1.y, p2.y);
                let x = p1.x;
                for j in low..=high {
                    grid[x as usize][j as usize] = true;
                }
            } else {
                let low = min(p1.x, p2.x);
                let high = max(p1.x, p2.x);
                for j in low..=high {
                    grid[j as usize][p1.y as usize] = true;
                }
            }
        }
    }

    let mut currx:usize = 500;
    let mut curry: usize = 0;

    let mut result = 0;
    loop {
        let mut stopped = false;
        loop {
            if !grid[currx][curry+1] {
                curry += 1;
            } else if !grid[currx-1][curry+1] {
                currx -= 1;
                curry += 1;
            } else if !grid[currx+1][curry+1] {
                currx += 1;
                curry += 1;
            } else {
                grid[currx][curry] = true;
                stopped = true;
                break;
            }
        }
        result += 1;
        if currx == 500 && curry == 0 {
            break;
        }
        currx = 500 as usize;
        curry = 0;
    }

    println!("result: {}", result);
}

struct Point {
    x: u32,
    y: u32,
}

fn parse_line(s: &String) -> Vec<Point> {
    let mut result = Vec::new();
    let parts = s.split(' ').collect::<Vec<&str>>();
    for i in (0..parts.len()).step_by(2) {
        let xy = parts[i].split(',').collect::<Vec<&str>>();
        result.push(Point{ x: xy[0].parse().unwrap(), y: xy[1].parse().unwrap() })
    }
    return result;
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