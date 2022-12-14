use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Top3 {
    first: u32,
    second: u32,
    third: u32,
}

impl Top3 {
    fn update(&mut self, x: u32) {
        if x > self.first {
            self.third = self.second;
            self.second = self.first;
            self.first = x;
        } else if x > self.second {
            self.third = self.second;
            self.second = x;
        } else if x > self.third {
            self.third = x;
        }
    }

    fn sum(&self) -> u32 {
        return self.first + self.second + self.third;
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut top = Top3 {
            first: 0, 
            second: 0,
            third: 0,
        };
        let mut curr = 0u32;
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    top.update(curr);
                    curr = 0;
                } else {
                   let cal: u32 = ip.parse().unwrap();
                   curr += cal;
                }
            }
        }
        top.update(curr);
        println!("top 3: {}  {}  {}", top.first, top.second, top.third);
        println!("sum: {}", top.sum());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}