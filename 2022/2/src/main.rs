use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Match {
    p1: char,
    p2: char,
}

impl Match {
    fn new(a: char, b: char) -> Match {
        let p1 = char_to_rock_paper_scissors(a);
        let p2 = result_to_rock_paper_scissors(p1, b);
        return Match { 
            p1: p1,
            p2: p2,
        }
    }

    fn score(&self) -> u32 {
        // score for shape selected
        let mut score: u32 = match self.p2 {
            'R' => 1,
            'P' => 2,
            'S' => 3,
            _ => 0,
        };

        // score loss/draw/win
        score += match self.p1 {
            'R' => match self.p2 {
                'R' => 3,
                'P' => 6,
                'S' => 0,
                _ => 0,
            },
            'P' => match self.p2 {
                'R' => 0,
                'P' => 3,
                'S' => 6,
                _ => 0,
            },
            'S' => match self.p2 {
                'R' => 6,
                'P' => 0,
                'S' => 3,
                _ => 0,
            },
            _ => 0,
        };
        return score;
    }
}

fn char_to_rock_paper_scissors(c: char) -> char {
    match c {
        'A' | 'X' => 'R',
        'B' | 'Y' => 'P',
        'C' | 'Z' => 'S',
        _ => panic!(),
    }
}

fn result_to_rock_paper_scissors(a: char, result: char) -> char {
    match a {
        'R' => match result {
            'X' => 'S',
            'Y' => 'R',
            'Z' => 'P',
            _ => '0',
        },
        'P' => match result {
            'X' => 'R',
            'Y' => 'P',
            'Z' => 'S',
            _ => '0',
        },
        'S' => match result {
            'X' => 'P',
            'Y' => 'S',
            'Z' => 'R',
            _ => '0',
        },
        _ => '0',
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum_of_scores: u32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let p1 = ip.as_bytes()[0];
                let p2 = ip.as_bytes()[2] as char;
                sum_of_scores += Match::new(p1 as char, p2 as char).score();
            }
        }
        println!("sum: {}", sum_of_scores);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}