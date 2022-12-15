use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::path::PathBuf;

fn main() {
    let lines = read_file("input.txt");
    let mut dirs: HashMap<String, u32> = HashMap::new();

    let mut dir = PathBuf::from("/");
    walk_tree(&mut dir, 0, &lines, &mut dirs);

    let mut total_sizes: HashMap<String, u32> = HashMap::new();
    for (dir, size) in dirs {
        let p = PathBuf::from(dir);
        for anc in p.ancestors() {
            total_sizes.entry(String::from(anc.to_str().unwrap())).and_modify(|v| *v += size).or_insert(size);
        }
    }

    let mut result = 0;
    for (_, size) in total_sizes.iter() {
        if *size < 100000 {
            result += *size;
        }
    }
    println!("result: {}", result);

    // part 2
    let used = *total_sizes.get(&String::from("/")).unwrap();
    let unused = 70_000_000 - used;
    let need_unused = 30_000_000 - unused;

    let mut min: u32 = 70_000_000;
    for (_, size) in total_sizes.iter() {
        if *size >= need_unused && *size < min {
            min = *size;
        }
    }
    println!("min size to delete: {}", min);
}

fn walk_tree(curr: &mut PathBuf, i: usize, lines: &Vec<String>, m: &mut HashMap<String, u32>) { 
    if i >= lines.len() {
        return;
    }
    let parts = parse_line(&lines[i]);
    if parts[0] == "$" {
        if parts[1] == "cd" {
            if parts[2] == ".." {
                curr.pop();
            } else if parts[2] == "/" {
                *curr = PathBuf::from("/");
            } else {
                curr.push(parts[2]);
            }
            walk_tree(curr, i+1, lines, m);
        } else {
            // ls
            let mut j = i+1;
            let mut size = 0;
            while j < lines.len() {
                let parts = parse_line(&lines[j]);
                if parts[0] == "$" {
                    break;
                } else if parts[0] != "dir" {
                    size += parts[0].parse::<u32>().unwrap();
                }
                j += 1;
            }
            m.insert(String::from(curr.to_str().unwrap()), size);
            walk_tree(curr, j, lines, m)
        }
    } 
}

fn parse_line(s: &String) -> Vec<&str> {
    return s.split(' ').collect::<Vec<&str>>();
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