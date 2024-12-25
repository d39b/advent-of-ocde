use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let (grid, moves) = read_input("example.txt");
    // let (grid, moves) = read_input("example2.txt");
    let (grid, moves) = read_input("input.txt");
    part1(&grid, &moves);
    part2(&grid, &moves);
}

fn part1(grid: &Vec<Vec<u8>>, moves: &Vec<u8>) {
    let n = grid.len();
    let mut g = vec![vec![0; n]; n];
    let mut robot_r = 0i32;
    let mut robot_c = 0i32;
    for r in 0..n {
        for c in 0..n {
            g[r][c] = grid[r][c];
            if g[r][c] == b'@' {
                robot_r = r as i32;
                robot_c = c as i32;
            }
        }
    }

    for &m in moves {
        let mut dr: i32 = 0;
        let mut dc: i32 = 0;
        match m {
            b'>' => {
                dr = 0;
                dc = 1;
            }
            b'<' => {
                dr = 0;
                dc = -1;
            }
            b'^' => {
                dr = -1;
                dc = 0;
            }
            b'v' => {
                dr = 1;
                dc = 0;
            }
            _ => {}
        }

        let mut j = 0;
        let mut r = robot_r;
        let mut c = robot_c;
        let mut empty_found = false;
        loop {
            r += dr;
            c += dc;
            let x = g[r as usize][c as usize];
            if x == b'O' {
                j += 1;
            } else if x == b'#' {
                break;
            } else if x == b'.' {
                empty_found = true;
                break;
            }
        }
        if empty_found {
            for i in 0..j + 1 {
                g[r as usize][c as usize] = g[(r - dr) as usize][(c - dc) as usize];
                r -= dr;
                c -= dc;
            }
            g[robot_r as usize][robot_c as usize] = b'.';
            robot_r += dr;
            robot_c += dc;
        }
    }

    let mut result = 0;
    for r in 0..n {
        for c in 0..n {
            if g[r][c] == b'O' {
                result += r * 100 + c;
            }
        }
    }
    println!("{result}");
}

fn part2(grid: &Vec<Vec<u8>>, moves: &Vec<u8>) {
    let n = grid.len();
    let mut g = vec![vec![0; 2 * n]; n];
    let mut robot_r = 0i32;
    let mut robot_c = 0i32;
    for r in 0..n {
        for c in 0..n {
            match grid[r][c] {
                b'@' => {
                    robot_r = r as i32;
                    robot_c = 2 * c as i32;
                    g[r][2 * c] = b'@';
                    g[r][2 * c + 1] = b'.';
                }
                b'#' => {
                    g[r][2 * c] = b'#';
                    g[r][2 * c + 1] = b'#';
                }
                b'.' => {
                    g[r][2 * c] = b'.';
                    g[r][2 * c + 1] = b'.';
                }
                b'O' => {
                    g[r][2 * c] = b'[';
                    g[r][2 * c + 1] = b']';
                }
                _ => {}
            }
        }
    }

    // for r in 0..n {
    //     let s = String::from_utf8(g[r].clone()).unwrap();
    //     println!("{s}");
    // }
    // println!();

    for &m in moves {
        let mut dr: i32 = 0;
        let mut dc: i32 = 0;
        let mut horiz = false;
        match m {
            b'>' => {
                // println!("move > right");
                dr = 0;
                dc = 1;
                horiz = true;
            }
            b'<' => {
                // println!("move < left");
                dr = 0;
                dc = -1;
                horiz = true;
            }
            b'^' => {
                // println!("move ^ up");
                dr = -1;
                dc = 0;
            }
            b'v' => {
                // println!("move v down");
                dr = 1;
                dc = 0;
            }
            _ => {}
        }

        if horiz {
            let mut j = 0;
            let mut r = robot_r;
            let mut c = robot_c;
            let mut empty_found = false;
            loop {
                r += dr;
                c += dc;
                let x = g[r as usize][c as usize];
                if x == b'[' || x == b']' {
                    j += 1;
                } else if x == b'#' {
                    break;
                } else if x == b'.' {
                    empty_found = true;
                    break;
                }
            }
            if empty_found {
                for i in 0..j + 1 {
                    g[r as usize][c as usize] = g[(r - dr) as usize][(c - dc) as usize];
                    r -= dr;
                    c -= dc;
                }
                g[robot_r as usize][robot_c as usize] = b'.';
                robot_r += dr;
                robot_c += dc;
            }
        } else {
            let mut hits_border = false;
            let mut boxes = vec![];
            let x = g[(robot_r + dr) as usize][(robot_c + dc) as usize];
            if x == b'[' {
                boxes.push((robot_r + dr, robot_c + dc));
            } else if x == b']' {
                boxes.push((robot_r + dr, robot_c + dc - 1));
            } else if x == b'#' {
                continue;
            } else if x == b'.' {
                g[(robot_r + dr) as usize][(robot_c + dc) as usize] = b'@';
                g[robot_r as usize][robot_c as usize] = b'.';
                robot_r += dr;
                robot_c += dc;
                continue;
            }
            let mut bi = 0;
            while bi < boxes.len() && !hits_border {
                let (r, c) = boxes[bi];
                bi += 1;
                let x = g[(r + dr) as usize][(c + dc) as usize];
                if x == b'[' {
                    boxes.push((r + dr, c + dc));
                } else if x == b']' {
                    boxes.push((r + dr, c + dc - 1));
                } else if x == b'#' {
                    hits_border = true;
                    break;
                }

                let x = g[(r + dr) as usize][(c + dc + 1) as usize];
                if x == b'[' {
                    boxes.push((r + dr, c + dc + 1));
                } else if x == b'#' {
                    hits_border = true;
                    break;
                }
            }
            if !hits_border {
                for &(r, c) in boxes.iter().rev() {
                    g[r as usize][c as usize] = b'.';
                    g[r as usize][(c + 1) as usize] = b'.';
                    g[(r + dr) as usize][(c + dc) as usize] = b'[';
                    g[(r + dr) as usize][(c + dc + 1) as usize] = b']';
                }
                g[(robot_r + dr) as usize][(robot_c + dc) as usize] = b'@';
                g[robot_r as usize][robot_c as usize] = b'.';
                robot_r += dr;
                robot_c += dc;
            }
        }

        // for r in 0..n {
        //     let s = String::from_utf8(g[r].clone()).unwrap();
        //     println!("{s}");
        // }
        // println!();
    }

    let mut result = 0;
    for r in 0..n {
        for c in 0..2 * n {
            if g[r][c] == b'[' {
                // let hd1 = c;
                // let hd2 = 2 * n - c;
                // let vd1 = r;
                // let vd2 = n - 1 - r;
                // if hd1 < hd2 {
                //     result += hd1;
                // } else {
                //     result += hd2;
                // }
                // if vd1 < vd2 {
                //     result += 100 * vd1;
                // } else {
                //     result += 100 * vd2;
                // }
                result += 100 * r + c;
            }
        }
    }
    println!("{result}");
}

fn read_input(f: &str) -> (Vec<Vec<u8>>, Vec<u8>) {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let mut grid = vec![];
    let mut moves = vec![];
    let mut reading_grid = true;
    for line in reader.lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            reading_grid = false;
        } else if reading_grid {
            grid.push(line.into_bytes());
        } else {
            moves.extend_from_slice(line.as_bytes());
        }
    }

    return (grid, moves);
}
