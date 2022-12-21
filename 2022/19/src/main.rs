use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::cmp::{min, max};

fn main() {
    let lines = read_file("input2.txt");

    let mut result = 1;
    for li in 0..3 {
        let bp = parse_line(&lines[li]);
        let mut curr: Vec<State> = Vec::new();
        curr.push(State::initial());
        let mut max_geodes = 0;
        let mut max_geodes_robots = 0;
        for i in 0..32 {
            let mut next: Vec<State> = Vec::new();
            for s in curr.iter() {
                for nx in s.compute_next(&bp) {
                    if is_valid(&nx, i, max_geodes, max_geodes_robots) {
                        next.push(nx);
                    }
                }
            }
            for nx in next.iter() {
                if nx.geode > max_geodes {
                    max_geodes = nx.geode;
                    max_geodes_robots = nx.geode_robots;
                } else if nx.geode == max_geodes {
                    if nx.geode_robots > max_geodes_robots {
                        max_geodes_robots = nx.geode_robots;
                    }
                }
            }

            // if a and b are states and every element of the state is less in a than in b
            // i.e. number of ore, clay, obsidian, geodes resources and the number of robots for each type
            // then we can remove state a
            // there are usually quite a few of these
            // however checking all the possible pairs eventually becomes too expensive, so we check at most ~2M 
            let mut can_remove = HashSet::new();
            for j in 0..min(next.len()-1, 2000) {
                for k in j+1..min(next.len(), 2000) {
                    if next[j].less_or_eq(&next[k]) {
                        can_remove.insert(j);
                    } else if next[k].less_or_eq(&next[j]) {
                        can_remove.insert(k);
                    }
                }
            }
            let mut filtered = Vec::new();
            for j in 0..next.len() {
                if !can_remove.contains(&j) {
                    filtered.push(next[j].clone());
                }
            }
            curr = filtered;

        }
        result *= max_geodes;
    }

    println!("result: {}", result);
}

// a state is only further expanded if this function returns true
// we can eliminate states that can e.g. not overtake the current best state
fn is_valid(s: &State, i: u64, max_geodes: u64, max_geodes_robots: u64) -> bool {
    if s.geode >= max_geodes && s.geode_robots >= max_geodes_robots {
        return true;
    }
    let remaining_steps = 31-i;
    // this is the number of geodes the current best state will at least end up with
    // i.e. if no further geode robots are made
    let min_max = max_geodes + remaining_steps*max_geodes_robots;
    // this is the maximum number of geodes state s can possibly end up with
    // i.e. if a geode making robot is made at every remaining step
    let mut max_max = s.geode;
    if remaining_steps <= 2 {
        max_max += s.geode + remaining_steps*s.geode_robots;
    } else {
        max_max = s.geode + remaining_steps*s.geode_robots + (remaining_steps-1)*(remaining_steps-2)/2;
    }
    if max_max < min_max {
        return false;
    }
    return true;
}

struct Blueprint {
    ore: u64,
    clay: u64,
    obs: (u64, u64),
    geode: (u64, u64),
}

impl Blueprint {
    fn max_ore(&self) -> u64 {
        return max(max(self.ore, self.clay), max(self.obs.0, self.geode.0));
    }
}

#[derive(Clone)]
struct State {
    ore: u64,
    clay: u64,
    obs: u64,
    geode: u64,
    ore_robots: u64,
    clay_robots: u64,
    obs_robots: u64,
    geode_robots: u64,
}

impl State {
    fn initial() -> State {
        return State{
            ore: 0,
            clay: 0,
            obs: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obs_robots: 0,
            geode_robots: 0,
        };
    }

    fn print(&self) {
        println!("{} {} {} {} | {} {} {} {}", self.ore, self.clay, self.obs, self.geode, self.ore_robots, self.clay_robots, self.obs_robots, self.geode_robots);
    }

    fn value_sum(&self) -> u64 {
        return self.ore + self.clay + self.obs + self.geode + self.ore_robots + self.clay_robots + self.obs_robots + self.geode_robots;
    }

    fn less_or_eq(&self, other: &State) -> bool {
        if self.ore > other.ore {
            return false;
        }
        if self.clay > other.clay {
            return false;
        }
        if self.obs > other.obs {
            return false;
        }
        if self.geode > other.geode {
            return false;
        }
        if self.ore_robots > other.ore_robots {
            return false;
        }
        if self.clay_robots > other.clay_robots {
            return false;
        }
        if self.obs_robots > other.obs_robots {
            return false;
        }
        if self.geode_robots > other.geode_robots {
            return false;
        }
        return true;
    }

    fn can_make_ore_robot(&self, bp: &Blueprint) -> bool {
        return self.ore >= bp.ore;
    }

    fn can_make_clay_robot(&self, bp: &Blueprint) -> bool {
        return self.ore >= bp.clay;
    }

    fn can_make_obs_robot(&self, bp: &Blueprint) -> bool {
        return self.ore >= bp.obs.0 && self.clay >= bp.obs.1;
    }

    fn can_make_geode_robot(&self, bp: &Blueprint) -> bool {
        return self.ore >= bp.geode.0 && self.obs >= bp.geode.1;
    }

    // return the number of different robots there are
    fn robot_types(&self) -> u64 {
        let mut result = 1;
        if self.clay_robots > 0 {
            result += 1;
        }
        if self.obs_robots > 0 {
            result += 1;
        }
        if self.geode_robots > 0 {
            result += 1;
        }
        return result;
    }

    fn compute_next(&self, bp: &Blueprint) -> Vec<State> {
        let mut result = Vec::new();
        let with_resources = self.add_resources();
        let mut can_make = 0;
        let robot_types = self.robot_types();

        // if we already make more ore per minute than the maximum ore cost of any robot
        // it does not make sense to make another ore robot
        if self.ore_robots < bp.max_ore() && self.can_make_ore_robot(bp) {
            result.push(with_resources.with_ore_robot(bp));
            can_make += 1;
        }

        if self.can_make_clay_robot(bp) {
            result.push(with_resources.with_clay_robot(bp));
            can_make += 1;
        }
        
        if self.can_make_obs_robot(bp) {
            result.push(with_resources.with_obs_robot(bp));
            can_make += 1;
        }

        if self.can_make_geode_robot(bp) {
            result.push(with_resources.with_geode_robot(bp));
            can_make += 1;
        }

        // if we have just ore robots 
        // but could make an ore or clay robot
        // it does not make sense to do nothing
        // i.e. can_make=2  robot_types=1
        // if we have ore and clay robots
        // but could make ore/clay/obs
        // it does not make sense to do nothing
        // can_make=3, robot_types=2
        // same if we have ore/clay/obs (and possibly geode)
        // but could make ore/clay/obs/geode
        // can_make=4, robot_types=3/4
        if can_make < 4 && (can_make <= robot_types) {
            result.push(with_resources);
        }

        return result;
    }

    fn add_resources(&self) -> State {
        return State{
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obs: self.obs + self.obs_robots,
            geode: self.geode + self.geode_robots,
            ..*self
        };
    }

    fn with_ore_robot(&self, bp: &Blueprint) -> State {
        return State { 
            ore_robots: self.ore_robots + 1,
            ore: self.ore - bp.ore,
            ..*self
        };
    }

    fn with_clay_robot(&self, bp: &Blueprint) -> State {
        return State { 
            clay_robots: self.clay_robots + 1,
            ore: self.ore - bp.clay,
            ..*self
        };
    }

    fn with_obs_robot(&self, bp: &Blueprint) -> State {
        return State { 
            obs_robots: self.obs_robots + 1,
            ore: self.ore - bp.obs.0,
            clay: self.clay - bp.obs.1,
            ..*self
        };
    }

    fn with_geode_robot(&self, bp: &Blueprint) -> State {
        return State { 
            geode_robots: self.geode_robots + 1,
            ore: self.ore - bp.geode.0,
            obs: self.obs - bp.geode.1,
            ..*self
        };
    }
}

fn parse_line(s: &String) -> Blueprint {
    let parts = s.split(' ').collect::<Vec<&str>>();
    let mut nums = Vec::new();
    for p in parts.iter() {
        let x = p.parse::<u64>();
        if !x.is_err() {
            nums.push(x.unwrap());
        }
    }
    return Blueprint {
        ore: nums[0],
        clay: nums[1],
        obs: (nums[2], nums[3]),
        geode: (nums[4], nums[5]),
    }
}

fn read_file(f: &str) -> Vec<String> {
    return BufReader::new(File::open(f).unwrap())
        .lines()
        .map(|r| {
            if let Ok(s) = r {
                return s;
            } else {
                panic!();
            }
        })
        .collect::<Vec<String>>();
}
