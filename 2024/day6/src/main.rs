use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let grid = read_input("example.txt");
    let grid = read_input("input.txt");
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<u8>>) {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut r = 0;
    let mut c = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'^' {
                r = i as i32;
                c = j as i32;
                break;
            }
        }
    }
    let mut dir = Direction::Up;
    visited[r as usize][c as usize] = true;
    loop {
        let (dr, dc) = dir.coordinate_delta();
        let nr = r + dr;
        let nc = c + dc;
        // guard left the room
        if nr < 0 || nc < 0 || nr >= (m as i32) || nc >= (n as i32) {
            break;
        }
        if grid[nr as usize][nc as usize] == b'#' {
            dir = dir.turn90();
        } else {
            r = nr;
            c = nc;
            visited[nr as usize][nc as usize] = true;
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if visited[i][j] {
                result += 1;
            }
        }
    }
    println!("{result}");
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn coordinate_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn90(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

// we could just simulate the path the guard takes for every possible placement of an obstacle
// but we can speed it up a bit by pre-computing the steps to the next obstacle
fn part2(grid: &Vec<Vec<u8>>) {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut sr = 0;
    let mut sc = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'^' {
                sr = i as i32;
                sc = j as i32;
                break;
            }
        }
    }
    let mut up = vec![vec![(-1, -1); n]; m];
    let mut down = vec![vec![(-1, -1); n]; m];
    let mut left = vec![vec![(-1, -1); n]; m];
    let mut right = vec![vec![(-1, -1); n]; m];
    for c in 0..n {
        if grid[0][c] == b'#' {
            up[0][c] = (0, c as i32);
        }
        if grid[m - 1][c] == b'#' {
            down[m - 1][c] = ((m - 1) as i32, c as i32);
        }
    }
    for r in 1..m {
        for c in 0..n {
            if grid[r][c] == b'#' {
                up[r][c] = (r as i32, c as i32);
            } else {
                up[r][c] = up[r - 1][c];
            }
            if grid[m - 1 - r][c] == b'#' {
                down[m - 1 - r][c] = ((m - 1 - r) as i32, c as i32);
            } else {
                down[m - 1 - r][c] = down[m - r][c];
            }
        }
    }
    for r in 0..m {
        if grid[r][0] == b'#' {
            left[r][0] = (r as i32, 0);
        }
        if grid[r][n - 1] == b'#' {
            right[r][n - 1] = (r as i32, (n - 1) as i32);
        }
    }
    for c in 1..n {
        for r in 0..m {
            if grid[r][c] == b'#' {
                left[r][c] = (r as i32, c as i32);
            } else {
                left[r][c] = left[r][c - 1];
            }
            if grid[r][n - 1 - c] == b'#' {
                right[r][n - 1 - c] = (r as i32, (n - 1 - c) as i32);
            } else {
                right[r][n - 1 - c] = right[r][n - c];
            }
        }
    }

    let mut result = 0;
    for or in 0..(m as i32) {
        for oc in 0..(n as i32) {
            if grid[or as usize][oc as usize] == b'#' || (or == sr && oc == sc) {
                continue;
            }
            let mut r = sr;
            let mut c = sc;
            let mut dir = Direction::Up;
            let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
            visited.insert((r, c, dir.clone()));
            let mut has_cycle = false;
            loop {
                let (nr, nc) = match dir {
                    Direction::Up => {
                        let (mut nr, mut nc) = up[r as usize][c as usize];
                        if oc == c {
                            if or < r && (or > nr || nr == -1) {
                                nc = c;
                                nr = or;
                            }
                        }
                        (nr, nc)
                    }
                    Direction::Down => {
                        let (mut nr, mut nc) = down[r as usize][c as usize];
                        if oc == c {
                            if or > r && (or < nr || nr == -1) {
                                nc = c;
                                nr = or;
                            }
                        }
                        (nr, nc)
                    }
                    Direction::Left => {
                        let (mut nr, mut nc) = left[r as usize][c as usize];
                        if or == r {
                            if oc < c && (oc > nc || nc == -1) {
                                nr = r;
                                nc = oc;
                            }
                        }
                        (nr, nc)
                    }
                    Direction::Right => {
                        let (mut nr, mut nc) = right[r as usize][c as usize];
                        if or == r {
                            if oc > c && (oc < nc || nc == -1) {
                                nr = r;
                                nc = oc;
                            }
                        }
                        (nr, nc)
                    }
                };
                if nr == -1 && nc == -1 {
                    break;
                }
                r = match dir {
                    Direction::Up => nr + 1,
                    Direction::Down => nr - 1,
                    Direction::Left => nr,
                    Direction::Right => nr,
                };
                c = match dir {
                    Direction::Up => nc,
                    Direction::Down => nc,
                    Direction::Left => nc + 1,
                    Direction::Right => nc - 1,
                };
                dir = dir.turn90();
                if visited.contains(&(r, c, dir.clone())) {
                    has_cycle = true;
                    break;
                } else {
                    visited.insert((r, c, dir.clone()));
                }
            }
            if has_cycle {
                result += 1;
            }
        }
    }
    println!("{result}");
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let result = reader.lines().map(|x| x.unwrap().into_bytes()).collect();
    return result;
}
