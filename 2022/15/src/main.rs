use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::RangeBounds;
use std::vec::Vec;
use std::cmp::{min, max};
use std::option::Option;

fn main() {
    let lines = read_file("input.txt");

    let n = lines.len();

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in lines.iter() {
        let (sensor, beacon) = parse_line(line);
        sensors.push(sensor);
        beacons.push(beacon);
    }

/* 
    // part 1
    let target_y = 2_000_000;


    let mut result: HashMap<i64, bool> = HashMap::new();

    for i in 0..n {
        let sensor = &sensors[i];
        let beacon = &beacons[i];
        let dist = sensor.man_dist(beacon);
        let points = sensor.find_points_with_dist_less_than(target_y, dist);
        for j in points {
            result.insert(j, true);
        }
    }

    for beacon in beacons.iter() {
        if beacon.y == target_y {
            result.remove(&beacon.x);
        }
    }

    println!("result: {}", result.len());
 */

    // part 2
    // (x,y) of the beacon we are looking for satisfies 0 <= x,y <= 4000000
    // and the distance from any sensor to it must be higher than the distance to the sensor's closest beacon
    // there is only one possible position for the distress beacon
    // can we maybe eliminate entire ranges of x/y coordinates?
    // that is we try to find x and y separately

    // for every x (or y)
    //      iterate over every sensor/beacon pair
    //          can compute the range of possible y values
    //              this can be computed in constant time something like [1,x]
    //                  this will yield 20 values
    //                      we can sort these, if there is a solution there must be two intervals such that [i1,i2] [i2+2][i3]

    //let maxx = 20;
    let maxx = 4_000_000;
    for x in 0..maxx+1 {
        let mut invs = Vec::new();
        for i in 0..n {
            let sensor = &sensors[i];
            let beacon = &beacons[i];
            let dist = sensor.man_dist(beacon);
            match sensor.find_points_with_dist_less_than_interval(x, dist) {
                Some(x) => invs.push(x),
                _ => (),
            }
        }

        invs.sort_by(|x,y| {
            if x.a < y.a {
                std::cmp::Ordering::Less
            } else if x.a > y.a {
                std::cmp::Ordering::Greater
            } else {
                if x.b < y.b {
                    std::cmp::Ordering::Less
                } else if x.b > y.b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });

        // there might still be some intervals that are contained in one another
        // need to get rid of these, e.g. [-2, 16], [14, 14], [16, 16]
        let mut invs_diff: Vec<Interval> = Vec::new();
        let mut ii = 0;
        while ii < invs.len() {
            let mut nooverlap = true;
            if ii > 0 && invs[ii-1].contains(&invs[ii]) {
                nooverlap = false;
            }
            if ii+1 < invs.len() && !invs[ii+1].eq(&invs[ii]) && invs[ii+1].contains(&invs[ii]) {
                nooverlap = false;
            }
            if !invs_diff.is_empty() && invs_diff.last().unwrap().contains(&invs[ii]) {
                nooverlap = false;
            }
            if nooverlap {
                invs_diff.push(invs[ii].clone());
            }
            ii += 1;
        }

        for i in 0..invs_diff.len()-1 {
            let i1 = &invs_diff[i];
            let i2 = &invs_diff[i+1];
            if i1.b + 2 == i2.a {
                println!("solution: {}, {}, {}", x, i1.b+1, x*4_000_000+i1.b+1);
            }
        }
    }
}

#[derive(Debug,Clone,Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug,Clone,Copy)]
struct Interval {
    a: i64,
    b: i64,
}

impl Interval {
    fn contains(&self, other: &Interval) -> bool {
        if self.a <= other.a && self.b >= other.b {
            return true;
        }
        return false;
    }

    fn eq(&self, other: &Interval) -> bool {
        return self.a == other.a && self.b == other.b;
    }
}

impl Point {
    fn man_dist(&self, other: &Point) -> i64 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }

    fn find_points_with_dist_less_than(&self, y: i64, dist: i64) -> Vec<i64> {
        let mut result = Vec::new();
        let available_x_dist = dist - (y-self.y).abs();
        if available_x_dist >= 0 {
            result.push(self.x);
            for i in 1..available_x_dist+1 {
                result.push(self.x-i);
                result.push(self.x+i);
            }
        }
        return result;
    }

    fn find_points_with_dist_less_than_interval(&self, x: i64, dist: i64) -> Option<Interval> {
        let available_y_dist = dist - (x-self.x).abs();
        if available_y_dist >= 0 {
            return Option::Some(Interval { a: self.y - available_y_dist, b: self.y + available_y_dist });
        }
        return Option::None;
    }
}

fn parse_line(s: &String) -> (Point, Point) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    let p1 = Point{ x: parse_num(parts[2]), y: parse_num(parts[3]) };
    let p2 = Point{ x: parse_num(parts[8]), y: parse_num(parts[9]) };
    return (p1, p2);
}

fn parse_num(s: &str) -> i64 {
    let parts = s.trim_end_matches(|c| c == ',' || c == ':').split('=').collect::<Vec<&str>>();
    return parts[1].parse().unwrap();
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