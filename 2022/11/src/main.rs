use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input2.txt");

    let mut monkeys: Vec<Monkey> = Vec::new();
    // part 1 only 20 round
    // let n_rounds = 20;
    // part 2
    let n_rounds = 10000;
    // NOTE: this is a bit brittle, make sure input has 2 blank lines at the end
    let n_monkeys = lines.len() / 7;
    println!("number of monkeys: {}", n_monkeys);

    let mut items: Vec<WorryLevel> = Vec::new();

    for i in (0..lines.len()).step_by(7) {
        let start_index = items.len();
        items.append(&mut parse_items(&lines[i+1], n_monkeys));

        monkeys.push(Monkey{
            items: (start_index..items.len()).collect(),
            op: parse_operation(&lines[i+2]),
            divisible: parse_last_word(&lines[i+3]),
            true_monkey: parse_last_word(&lines[i+4]),
            false_monkey: parse_last_word(&lines[i+5]),
            items_inspected: 0,
        })
    }

    let mut mods: Vec<u64> = Vec::new();
    for monkey in monkeys.iter() {
        mods.push(monkey.divisible);
    }

    // to keep the numbers from exploding in part 2
    // we keep the worry level for each item modulo all the possible
    // numbers the monkeys divide by

    for _ in 0..n_rounds {
        for j in 0..n_monkeys {
            let monkey = &mut monkeys[j];
            monkey.items_inspected += monkey.items.len() as u64;
            let mut item_moves: Vec<ItemMove> = Vec::new();

            for item in monkey.items.iter() {
                // in part 1 we divide worry levels by 3
                // not in part 2
                //let worry_level = monkey.op.compute_new(*item) / 3;
                items[*item].apply_op(&monkey.op, &mods);
                if (items[*item].l[j]) % monkey.divisible == 0 {
                    item_moves.push(ItemMove{ to: monkey.true_monkey, index: *item });
                } else {
                    item_moves.push(ItemMove{ to: monkey.false_monkey, index: *item });
                }
            }

            monkey.items.clear();

            for im in item_moves.iter() {
                monkeys[im.to as usize].items.push(im.index);
            }
        }
    }

    let mut max = 0;
    let mut max2 = 0;

    for monkey in monkeys.iter() {
        if monkey.items_inspected > max {
            max2 = max;
            max = monkey.items_inspected;
        } else if monkey.items_inspected > max2 {
            max2 = monkey.items_inspected;
        }
    }

    println!("solution: {}", max*max2);
}

struct WorryLevel {
    l: Vec<u64>,
}

impl WorryLevel {
    fn apply_op(&mut self, op: &Op, mods: &Vec<u64>) {
        for i in 0..self.l.len() {
            let old = self.l[i];
            let m = mods[i];
            self.l[i] = match *op {
               Op::MultOld => (old * old) % m, 
               Op::AddOld => (old + old ) % m,
               Op::Mult(x) => (old * x) % m,
               Op::Add(x) => (old + x) % m,
            }
        }
    }
}

struct ItemMove {
    to: u64,
    index: usize,
}

struct Monkey {
    op: Op,
    divisible: u64,
    true_monkey: u64,
    false_monkey: u64,
    items: Vec<usize>,
    items_inspected: u64,
}

enum Op {
    MultOld,
    AddOld,
    Mult(u64),
    Add(u64),
}

fn parse_operation(s: &String) -> Op {
    let parts = s.split('=').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!();
    }
    let parts =  parts[1].trim().split(' ').collect::<Vec<&str>>();
    if parts.len() != 3 {
        panic!();
    }
    if parts[1] == "*" {
        if parts[2] == "old" {
            return Op::MultOld;
        } else {
            return Op::Mult(parts[2].parse().unwrap());
        }
    } else {
        if parts[2] == "old" {
            return Op::AddOld;
        } else {
            return Op::Add(parts[2].parse().unwrap());
        }
    } 
}

fn parse_items(s: &String, n_monkeys: usize ) -> Vec<WorryLevel> {
    let parts = s.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        panic!();
    }
    return parts[1].trim().split(", ").map(|x| -> WorryLevel { WorryLevel { l: vec![x.trim().parse().unwrap(); n_monkeys] } }).collect::<Vec<WorryLevel>>();
}

fn parse_last_word(s: &String) -> u64 {
    let parts = s.split(' ').collect::<Vec<&str>>();
    if parts.len() < 2 {
        panic!();
    }
    return parts.last().unwrap().parse().unwrap();
}

// fn print_state(monkeys: &Vec<Monkey>) {
//     for i in 0..monkeys.len() {
//         let mut s = format!("Monkey {}: ", i);
//         for item in &monkeys[i].items {
//             s.push_str(&format!("{}, ", *item));
//         }
//         println!("{}", s);
//     }
// }

fn read_file(f: &str) -> Vec<String> {
    return BufReader::new(File::open(f).unwrap()).lines().map(|r| {
        if let Ok(s) = r {
            return s;
        } else {
            panic!();
        }
    }).collect::<Vec<String>>();
}