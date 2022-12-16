use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");
    let n = lines.len();
    let m = lines[0].len();

    // part 1
    let mut visible = vec![vec![false; m]; n] ;

    // check rows   
    for i in 0..n {
        let mut max = 0;
        let mut maxRev = 0;
        for j in 0..m {
            let h = lines[i][j] - 48;
            if h > max || j == 0 {
                visible[i][j] = true;
                max = h;
            }

            let h = lines[i][m-1-j] - 48;
            if h > maxRev || j == 0 {
                visible[i][m-1-j] = true;
                maxRev = h;
            }
        }
    }

    // check columns
    for j in 0..m {
        let mut max = 0;
        let mut maxRev = 0;
        for i in 0..n {
            let h = lines[i][j] - 48;
            if h > max || i == 0 {
                visible[i][j] = true;
                max = h;
            }

            let h = lines[n-1-i][j] - 48;
            if h > maxRev || i == 0 {
                visible[n-1-i][j] = true;
                maxRev = h;
            }
        }
    }

    let mut result = 0;
    for i in 0..n {
        for j in 0..m {
            if visible[i][j] {
                result += 1;
            }
        }
    }
    println!("visible: {}", result);

    // part 2

    // since we multiply the viewing distances together, a tree on the edge
    // will always hve scenic score 0, so they can't be the solution

    // we can do 4 passes again, one for each direction to look in
    // when we are at a certain pos (i,j) with height k we want to find the closest pos
    // in the current direction with height >= k

    // scenic scores
    // init all values with 1
    let mut ss = vec![vec![1u32; m]; n] ;

    // check rows   
    for i in 0..n {
        let mut pos: [i32; 10] = [-1; 10];
        let mut posRev: [i32; 10] = [-1; 10];

        for j in 1..m {
            let h = lines[i][j] - 48;
            ss[i][j] *= viewing_dist(&pos, h, j);
            pos[h as usize] = j as i32;

            let h = lines[i][m-1-j] - 48;
            ss[i][m-1-j] *= viewing_dist_rev(&posRev, h, m-1-j,m-1);
            posRev[h as usize] = (m-1-j) as i32;
        }
    }

    // check columns
    for j in 0..m {
        let mut pos: [i32; 10] = [-1; 10];
        let mut posRev: [i32; 10] = [-1; 10];

        for i in 1..n {
            let h = lines[i][j] - 48;
            ss[i][j] *= viewing_dist(&pos, h, i);
            pos[h as usize] = i as i32;

            let h = lines[n-1-i][j] - 48;
            ss[n-1-i][j] *= viewing_dist_rev(&posRev, h, n-1-i, n-1);
            posRev[h as usize] = (n-1-i) as i32;
        }
    }

    let mut result = 0;
    // ignore trees on the edge of the grid
    for i in 1..n-1 {
        for j in 1..m-1 {
            if ss[i][j] > result {
                result = ss[i][j];
            }
        }
    }
    println!("max scenic score: {}", result);
}

fn viewing_dist(c: &[i32], t: u8, i: usize) -> u32 {
    let mut max: u32 = 0;
    for j in t..10 {
        if c[j as usize] > 0 {
            let index = c[j as usize] as u32;
            if index > max {
                max = index;
            }
        }
    } 
    return (i as u32) - max;
}

fn viewing_dist_rev(c: &[i32], t: u8, i: usize, n: usize) -> u32 {
    let mut min: u32 = n as u32;
    for j in t..10 {
        if c[j as usize] > 0 {
            let index = c[j as usize] as u32;
            if index < min {
                min = index;
            }
        }
    } 
    return min - (i as u32);
}

fn read_file(f: &str) -> Vec<Vec<u8>> {
    return BufReader::new(File::open(f).unwrap()).lines().map(|r| {
        if let Ok(s) = r {
            return s.as_bytes().to_vec();
        } else {
            panic!();
        }
    }).collect::<Vec<Vec<u8>>>();
}