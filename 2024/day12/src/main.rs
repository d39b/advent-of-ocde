use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let map = read_input("example.txt");
    let map = read_input("input.txt");
    part1(&map);
    part2(&map);
}

fn part1(map: &Vec<Vec<u8>>) {
    let mut result = 0;
    // maps are square
    let n = map.len();
    let mut visited = vec![vec![false; n]; n];
    for r in 0..n {
        for c in 0..n {
            if !visited[r][c] {
                let (area, perimeter) = dfs(r, c, map[r][c], &mut visited, map);
                result += area * perimeter;
            }
        }
    }
    println!("{result}");
}

fn dfs(r: usize, c: usize, p: u8, visited: &mut Vec<Vec<bool>>, map: &Vec<Vec<u8>>) -> (i32, i32) {
    visited[r][c] = true;
    let mut area = 1;
    let mut perimeter = 0;
    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nc < 0 || nr >= (map.len() as i32) || nc >= (map.len() as i32) {
            perimeter += 1;
            abc
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        if map[nr][nc] != p {
            perimeter += 1;
            continue;
        }
        if !visited[nr][nc] {
            let (na, np) = dfs(nr, nc, p, visited, map);
            area += na;
            perimeter += np;
        }
    }

    return (area, perimeter);
}

fn part2(map: &Vec<Vec<u8>>) {
    let mut result = 0;
    // maps are square
    let n = map.len();
    let mut visited = vec![vec![false; n]; n];
    for r in 0..n {
        for c in 0..n {
            if !visited[r][c] {
                let mut sides: Vec<Vec<(i32, i32)>> = vec![vec![]; 4];
                let area = dfs2(r, c, map[r][c], &mut visited, map, &mut sides);

                let mut side_count = 0;
                for side in sides.iter_mut() {
                    side.sort();
                    let (mut cr, mut cc) = side[0];
                    for &(nr, nc) in side[1..].iter() {
                        if nr == cr {
                            if nc != cc + 1 {
                                side_count += 1;
                            }
                        } else {
                            side_count += 1
                        }
                        cr = nr;
                        cc = nc;
                    }
                    side_count += 1;
                }

                result += side_count * area;
            }
        }
    }
    println!("{result}");
}

fn dfs2(
    r: usize,
    c: usize,
    p: u8,
    visited: &mut Vec<Vec<bool>>,
    map: &Vec<Vec<u8>>,
    sides: &mut Vec<Vec<(i32, i32)>>,
) -> i32 {
    visited[r][c] = true;
    let mut area = 1;
    for (dr, dc, side) in [(1, 0, 0), (-1, 0, 1), (0, 1, 2), (0, -1, 3)] {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nc < 0 || nr >= (map.len() as i32) || nc >= (map.len() as i32) {
            if side > 1 {
                sides[side].push((c as i32, r as i32));
            } else {
                sides[side].push((r as i32, c as i32));
            }
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        if map[nr][nc] != p {
            if side > 1 {
                sides[side].push((c as i32, r as i32));
            } else {
                sides[side].push((r as i32, c as i32));
            }
            continue;
        }
        if !visited[nr][nc] {
            let na = dfs2(nr, nc, p, visited, map, sides);
            area += na;
        }
    }
    return area;
}

fn read_input(f: &str) -> Vec<Vec<u8>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let result = reader.lines().map(|x| x.unwrap().into_bytes()).collect();
    return result;
}
