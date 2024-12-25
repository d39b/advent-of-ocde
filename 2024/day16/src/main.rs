use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let maze = read_input("example.txt");
    let maze = read_input("input.txt");
    part1(&maze);
    part2(&maze);
}

fn part1(maze: &Vec<Vec<u8>>) {
    // maze is square
    let n = maze.len();

    let mut sr = 0;
    let mut sc = 0;
    let mut er = 0;
    let mut ec = 0;
    for r in 0..n {
        for c in 0..n {
            if maze[r][c] == b'S' {
                sr = r;
                sc = c;
            } else if maze[r][c] == b'E' {
                er = r;
                ec = c;
            }
        }
    }

    // directions are north = 0, east = 1, south = 2, west = 3
    // we can turn 90° clockwise/anti-clockwise by adding 1 or -1 (mod 4)
    let mut q = vec![(sr, sc, 1, 0)];
    // could've used a vec but this is more convenient
    let mut d: HashMap<(usize, usize, i32), i32> = HashMap::new();
    d.insert((sr, sc, 1), 0);

    let mut qi = 0;
    while qi < q.len() {
        let (r, c, dir, s) = q[qi];
        qi += 1;
        if *(d.get(&(r, c, dir)).unwrap()) != s {
            continue;
        }
        let mut nr = 0;
        let mut nc = 0;
        match dir {
            0 => {
                nr = r - 1;
                nc = c;
            }
            1 => {
                nr = r;
                nc = c + 1;
            }
            2 => {
                nr = r + 1;
                nc = c;
            }
            3 => {
                nr = r;
                nc = c - 1;
            }
            _ => {}
        }
        if maze[nr][nc] != b'#' {
            if let Some(v) = d.get_mut(&(nr, nc, dir)) {
                if s + 1 < *v {
                    d.insert((nr, nc, dir), s + 1);
                    q.push((nr, nc, dir, s + 1));
                }
            } else {
                d.insert((nr, nc, dir), s + 1);
                q.push((nr, nc, dir, s + 1));
            }
        }
        let dir_turned = (dir + 1) % 4;
        if let Some(v) = d.get_mut(&(r, c, dir_turned)) {
            if s + 1000 < *v {
                d.insert((r, c, dir_turned), s + 1000);
                q.push((r, c, dir_turned, s + 1000));
            }
        } else {
            d.insert((r, c, dir_turned), s + 1000);
            q.push((r, c, dir_turned, s + 1000));
        }
        let dir_turned = (dir - 1 + 4) % 4;
        if let Some(v) = d.get_mut(&(r, c, dir_turned)) {
            if s + 1000 < *v {
                d.insert((r, c, dir_turned), s + 1000);
                q.push((r, c, dir_turned, s + 1000));
            }
        } else {
            d.insert((r, c, dir_turned), s + 1000);
            q.push((r, c, dir_turned, s + 1000));
        }
    }

    let mut result = -1;
    for dir in 0..4 {
        if let Some(v) = d.get(&(er, ec, dir)) {
            if result == -1 || *v < result {
                result = *v;
            }
        }
    }

    println!("{result}");
}

fn part2(maze: &Vec<Vec<u8>>) {
    // maze is square
    let n = maze.len();

    let mut sr = 0;
    let mut sc = 0;
    let mut er = 0;
    let mut ec = 0;
    for r in 0..n {
        for c in 0..n {
            if maze[r][c] == b'S' {
                sr = r;
                sc = c;
            } else if maze[r][c] == b'E' {
                er = r;
                ec = c;
            }
        }
    }

    // directions are north = 0, east = 1, south = 2, west = 3
    // we can turn 90° clockwise/anti-clockwise by adding 1 or -1 (mod 4)
    let mut q = vec![(sr, sc, 1, 0)];
    // could've used a vec but this is more convenient
    let mut d: HashMap<(usize, usize, i32), i32> = HashMap::new();
    let mut prev: HashMap<(usize, usize, i32), Vec<(usize, usize, i32)>> = HashMap::new();
    d.insert((sr, sc, 1), 0);

    let mut qi = 0;
    while qi < q.len() {
        let (r, c, dir, s) = q[qi];
        qi += 1;
        if *(d.get(&(r, c, dir)).unwrap()) != s {
            continue;
        }
        let mut nr = 0;
        let mut nc = 0;
        match dir {
            0 => {
                nr = r - 1;
                nc = c;
            }
            1 => {
                nr = r;
                nc = c + 1;
            }
            2 => {
                nr = r + 1;
                nc = c;
            }
            3 => {
                nr = r;
                nc = c - 1;
            }
            _ => {}
        }
        if maze[nr][nc] != b'#' {
            if let Some(v) = d.get_mut(&(nr, nc, dir)) {
                if s + 1 < *v {
                    d.insert((nr, nc, dir), s + 1);
                    q.push((nr, nc, dir, s + 1));
                    prev.insert((nr, nc, dir), vec![(r, c, dir)]);
                } else if s + 1 == *v {
                    if let Some(v) = prev.get_mut(&(nr, nc, dir)) {
                        v.push((r, c, dir));
                    }
                }
            } else {
                d.insert((nr, nc, dir), s + 1);
                q.push((nr, nc, dir, s + 1));
                prev.insert((nr, nc, dir), vec![(r, c, dir)]);
            }
        }
        let dir_turned = (dir + 1) % 4;
        if let Some(v) = d.get_mut(&(r, c, dir_turned)) {
            if s + 1000 < *v {
                d.insert((r, c, dir_turned), s + 1000);
                q.push((r, c, dir_turned, s + 1000));
                prev.insert((r, c, dir_turned), vec![(r, c, dir)]);
            } else if s + 1000 == *v {
                if let Some(v) = prev.get_mut(&(r, c, dir_turned)) {
                    v.push((r, c, dir));
                }
            }
        } else {
            d.insert((r, c, dir_turned), s + 1000);
            q.push((r, c, dir_turned, s + 1000));
            prev.insert((r, c, dir_turned), vec![(r, c, dir)]);
        }
        let dir_turned = (dir - 1 + 4) % 4;
        if let Some(v) = d.get_mut(&(r, c, dir_turned)) {
            if s + 1000 < *v {
                d.insert((r, c, dir_turned), s + 1000);
                q.push((r, c, dir_turned, s + 1000));
                prev.insert((r, c, dir_turned), vec![(r, c, dir)]);
            } else if s + 1000 == *v {
                if let Some(v) = prev.get_mut(&(r, c, dir_turned)) {
                    v.push((r, c, dir));
                }
            }
        } else {
            d.insert((r, c, dir_turned), s + 1000);
            q.push((r, c, dir_turned, s + 1000));
            prev.insert((r, c, dir_turned), vec![(r, c, dir)]);
        }
    }

    let mut result = -1;
    for dir in 0..4 {
        if let Some(v) = d.get(&(er, ec, dir)) {
            if result == -1 || *v < result {
                result = *v;
            }
        }
    }

    let mut p = vec![];
    for dir in 0..4 {
        if let Some(v) = d.get(&(er, ec, dir)) {
            if *v == result {
                p.push((er, ec, dir));
            }
        }
    }

    let mut on_shortest_path = HashSet::new();
    let mut pi = 0;
    while pi < p.len() {
        let (r, c, dir) = p[pi];
        pi += 1;
        on_shortest_path.insert((r, c));
        if let Some(v) = prev.get(&(r, c, dir)) {
            for &(nr, nc, dir2) in v.iter() {
                p.push((nr, nc, dir2));
            }
        }
    }

    println!("{result} {}", on_shortest_path.len());
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    return reader.lines().map(|x| x.unwrap().into_bytes()).collect();
}
