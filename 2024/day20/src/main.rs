use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let grid = read_input("input.txt");
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<u8>>) {
    let n = grid.len();

    let mut d = vec![vec![-1; n]; n];
    let mut er = 0;
    let mut ec = 0;
    for r in 0..n {
        let mut found = false;
        for c in 0..n {
            if grid[r][c] == b'E' {
                er = r;
                ec = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let mut r = er;
    let mut c = ec;
    d[r][c] = 0;
    while grid[r][c] != b'S' {
        let z = d[r][c];
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nr = (r as i32) + dr;
            let nc = (c as i32) + dc;
            if nr < 0 || nc < 0 || nr >= (n as i32) || nc >= (n as i32) {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if grid[nr][nc] != b'#' {
                if d[nr][nc] == -1 {
                    d[nr][nc] = z + 1;
                    r = nr;
                    c = nc;
                    break;
                }
            }
        }
    }

    let sr = r;
    let sc = c;
    let path_len = d[sr][sc];

    let mut result = 0;
    for r in 1..n - 1 {
        for c in 1..n - 1 {
            // it doesn't make sense to cheat when already at the end
            if grid[r][c] == b'#' || grid[r][c] == b'E' {
                continue;
            }
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nr = (r as i32) + dr;
                let nc = (c as i32) + dc;
                if nr < 0 || nc < 0 || nr >= (n as i32) || nc >= (n as i32) {
                    continue;
                }
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == b'#' {
                    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let xr = (nr as i32) + dr;
                        let xc = (nc as i32) + dc;
                        if xr < 0 || xc < 0 || xr >= (n as i32) || xc >= (n as i32) {
                            continue;
                        }
                        let xr = xr as usize;
                        let xc = xc as usize;
                        if xr == r && xc == c {
                            continue;
                        }
                        if grid[xr][xc] != b'#' {
                            let z = path_len - d[r][c] + 2 + d[xr][xc];
                            if path_len - z >= 100 {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{result}");
}

fn part2(grid: &Vec<Vec<u8>>) {
    let n = grid.len();

    let mut er = 0;
    let mut ec = 0;
    for r in 0..n {
        let mut found = false;
        for c in 0..n {
            if grid[r][c] == b'E' {
                er = r;
                ec = c;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let mut d: Vec<Vec<i32>> = vec![vec![-1; n]; n];
    let mut path = vec![];

    let mut r = er;
    let mut c = ec;

    d[r][c] = 0;
    path.push((r, c));
    while grid[r][c] != b'S' {
        let z = d[r][c];
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nr = (r as i32) + dr;
            let nc = (c as i32) + dc;
            if nr < 0 || nc < 0 || nr >= (n as i32) || nc >= (n as i32) {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if grid[nr][nc] != b'#' {
                if d[nr][nc] == -1 {
                    d[nr][nc] = z + 1;
                    path.push((nr, nc));
                    r = nr;
                    c = nc;
                    break;
                }
            }
        }
    }
    path.reverse();

    let path_len = d[r][c];

    let mut result = 0;
    for i in 0..path.len() - 1 {
        // a cheat is determined by its start and end position
        // it only makes sense for a cheat to go to a later position on the path
        let (r1, c1) = path[i];
        for j in i + 1..path.len() {
            let (r2, c2) = path[j];
            let mut dist = 0;
            if r1 >= r2 {
                dist += r1 - r2;
            } else {
                dist += r2 - r1;
            }
            if c1 >= c2 {
                dist += c1 - c2;
            } else {
                dist += c2 - c1;
            }

            if dist > 20 {
                continue;
            }

            let z = path_len - d[r1][c1] + (dist as i32) + d[r2][c2];
            if path_len - z >= 100 {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|x| x.unwrap().into_bytes())
        .collect::<Vec<Vec<u8>>>();
}
