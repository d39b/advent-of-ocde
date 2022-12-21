use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let mut nums = read_file("input.txt");
    let n = nums.len();

    // part 1
    // let mut s = (0..n).collect::<Vec<usize>>();
    // for i in 0..n {
    //     let dir = nums[i].signum();
    //     let c = nums[i].abs();
    //     let mut curr_index = 0;
    //     for j in 0..n {
    //         if s[j] == i {
    //             curr_index = j;
    //             break;
    //         }
    //     }
    //     if dir >= 0 {
    //         for j in 0..c {
    //             let a = curr_index % n;
    //             let b = (curr_index+1) % n;
    //             let tmp = s[b];
    //             s[b] = s[a];
    //             s[a] = tmp;
    //             curr_index = b;
    //         }
    //     } else {
    //         for j in 0..c {
    //             let a = curr_index % n;
    //             // TODO does this work correctly?
    //             let b = (curr_index + n - 1) % n;
    //             let tmp = s[b];
    //             s[b] = s[a];
    //             s[a] = tmp;
    //             curr_index = b;
    //         }
    //     }
    // }

    // let mut result = 0;
    // for j in 0..n {
    //     if nums[j] == 0 {
    //         for k in 0..n {
    //             if s[k] == j {
    //                 result += nums[s[(k+1000) % n]];
    //                 result += nums[s[(k+2000) % n]];
    //                 result += nums[s[(k+3000) % n]];
    //             }
    //         }
    //         break;
    //     }
    // }

    // println!("result: {}", result);

    // part 2
    // here the problem is that each number moves alot more often
    // however if a number is moved (n-1) times it is back at its old position
    // so we can just move a value  (value % (n-1)) times instead with the same result
    for i in 0..n {
        nums[i] *= 811589153;
    } 

    let mut s = (0..n).collect::<Vec<usize>>();
    for _ in 0..10 {
        for i in 0..n {
            let dir = nums[i].signum();
            let c = nums[i].abs() % ((n-1) as i64);
            let mut curr_index = 0;
            for j in 0..n {
                if s[j] == i {
                    curr_index = j;
                    break;
                }
            }
            if dir >= 0 {
                for j in 0..c {
                    let a = curr_index % n;
                    let b = (curr_index+1) % n;
                    let tmp = s[b];
                    s[b] = s[a];
                    s[a] = tmp;
                    curr_index = b;
                }
            } else {
                for j in 0..c {
                    let a = curr_index % n;
                    // TODO does this work correctly?
                    let b = (curr_index + n - 1) % n;
                    let tmp = s[b];
                    s[b] = s[a];
                    s[a] = tmp;
                    curr_index = b;
                }
            }
        }
    }

    let mut result = 0;
    for j in 0..n {
        if nums[j] == 0 {
            for k in 0..n {
                if s[k] == j {
                    result += nums[s[(k+1000) % n]];
                    result += nums[s[(k+2000) % n]];
                    result += nums[s[(k+3000) % n]];
                }
            }
            break;
        }
    }

    println!("result: {}", result);

}

fn read_file(f: &str) -> Vec<i64> {
    return BufReader::new(File::open(f).unwrap()).lines().map(|r| {
        if let Ok(s) = r {
            return s.parse().unwrap();
        } else {
            panic!();
        }
    }).collect::<Vec<i64>>();
}