use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input2.txt");

    let mut points = Vec::new();
    for line in lines.iter() {
        points.push(parse_line(line));
    }

    let mut result = 0;
    for i in 0..points.len() {
        let p = &points[i];
        let mut open_sides = 6;
        for j in 0..points.len() {
            if j != i {
                let o = &points[j];
                
                if p.touches(o) {
                    open_sides -= 1;
                }
            }
        }
        result += open_sides;
    }
    println!("result: {}", result);

    // part 2
    // exterior surface area
    // find all the trapped air cubes
    // we can do this by considering a fixed size volume that contains the entire
    // lava droplet, all the cubes of air on the outside of that colume are non-trapped
    // any cube of air directly connected to a non-trapped cube is also not trapped

    // inputs do not seems to contain any 0 coordinates

    let mut maxx = 0;
    let mut maxy = 0;
    let mut maxz = 0;
    for p in points.iter() {
        if p.x > maxx {
            maxx = p.x;
        }
        if p.y > maxy {
            maxy = p.y;
        }
        if p.z > maxz {
            maxz = p.z;
        }
    }

    let ix = Indexer{
        maxx: maxx+2,
        maxy: maxy+2,
        maxz: maxz+2,
    };
    let mut lava: HashMap<u64, bool> = HashMap::new();
    for p in points.iter() {
        lava.insert(ix.zip(p.x, p.y, p.z), true);
    }
    let mut not_trapped: HashMap<u64, bool> = HashMap::new();

    // this could be done more efficiently
    // to not need multiple iterations we would have to check the cubes from outside in
    // but this is annoying to code, so we just iterate until nothing changes anymore
    loop {
        let mut changed = false;
        for i in 0..(maxx+2)*(maxy+2)*(maxz+2) {
            if !lava.contains_key(&i) && !not_trapped.contains_key(&i) {
                let p = ix.unzip(i);

                // check if it is a cube on the edge
                if p.x == 0 || p.x == maxx+1 || p.y == 0 || p.y == maxy+1 || p.z == 0 || p.z == maxz+1 {
                    not_trapped.insert(i, true);
                    changed = true;
                    continue;
                }

                // check if a neighbouring cube is not trapped
                for nb in p.neighbours().iter() {
                    if not_trapped.contains_key(&ix.zip(nb.x, nb.y, nb.z)) {
                        not_trapped.insert(i, true);
                        changed = true;
                        break;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // subtract from result the sides that are touched by an air cube that is trapped
    for i in 0..(maxx+2)*(maxy+2)*(maxz+2) {
        if !lava.contains_key(&i) && !not_trapped.contains_key(&i) {
            let p = ix.unzip(i);
            for o in points.iter() {
                if p.touches(o) {
                    result -= 1;
                }
            }
        }
    }

    println!("result: {}", result);
}

struct Indexer {
    maxx: u64,
    maxy: u64,
    maxz: u64,
}

impl Indexer {
    fn zip(&self, x: u64, y: u64, z: u64) -> u64 {
        return z + y*self.maxz + x*(self.maxz * self.maxy);
    }

    fn unzip(&self, i: u64) -> Point {
        let z = i % self.maxz;
        let y = ((i - z) % (self.maxz * self.maxy)) / self.maxz;
        let x = (i-z-y) / (self.maxz * self.maxy);
        return Point { x: x, y: y, z: z }
    }
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point { 
    fn touches(&self, o: &Point) -> bool {
        let mut equal_coords = 0;
        let mut one_off_coords = 0;
        if self.x == o.x {
            equal_coords += 1;
        } else if self.x.abs_diff(o.x) == 1 {
            one_off_coords += 1;
        }
        if self.y == o.y {
            equal_coords += 1;
        } else if self.y.abs_diff(o.y) == 1 {
            one_off_coords += 1;
        }
        if self.z == o.z {
            equal_coords += 1;
        } else if self.z.abs_diff(o.z) == 1 {
            one_off_coords += 1;
        }
        
        if equal_coords == 2 && one_off_coords == 1 {
            return true;
        } else {
            return false;
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        return vec![
            Point { x: self.x+1, y: self.y, z: self.z },
            Point { x: self.x-1, y: self.y, z: self.z },
            Point { x: self.x, y: self.y+1, z: self.z },
            Point { x: self.x, y: self.y-1, z: self.z },
            Point { x: self.x, y: self.y, z: self.z+1 },
            Point { x: self.x, y: self.y, z: self.z-1 },
        ];
    }
}

fn parse_line(s: &String) -> Point {
    let parts = s.split(',').collect::<Vec<&str>>();
    return Point { 
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
        z: parts[2].parse().unwrap(),
    }
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