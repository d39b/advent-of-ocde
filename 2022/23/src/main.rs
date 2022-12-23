use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");

    let offset = 300;
    let (mut board, mut elves, n, m) = parse_state(&lines, offset);
    let n_elves = elves.len();
    let mut propositions = vec![Proposition::None; n_elves];

    let mut round = 0;
    loop {
        let mut propos_board = vec![vec![0; m]; n];
        let mut no_moves = 0;
        for j in 0..n_elves {
            let p = elves[j].propose(&board, round);
            match p {
                Proposition::Pos(y,x) => {
                    propos_board[y][x] += 1;
                }
                _ => no_moves += 1,
            }
            propositions[j] = p;
        }

        if no_moves == n_elves {
            break;
        }

        for j in 0..n_elves {
            let p = &propositions[j];
            match *p {
                Proposition::Pos(y,x) => {
                    if propos_board[y][x] == 1 {
                        let old_y = elves[j].y;
                        let old_x = elves[j].x;
                        board[old_y][old_x] = false;
                        board[y][x] = true;
                        elves[j].y = y;
                        elves[j].x = x;
                    }
                }
                _ => {},
            }
        }

        round += 1;
    }

    let mut min_x = 1<<30;
    let mut max_x = 0;
    let mut min_y = 1<<30;
    let mut max_y = 0;
    for i in 0..n {
        for j in 0..m {
            if board[i][j] {
                if i < min_y {
                    min_y = i;
                }
                if i > max_y {
                    max_y = i;
                }
                if j < min_x {
                    min_x = j;
                }
                if j > max_x {
                    max_x = j;
                }
            }
        }
    }

    println!("stopped after round: {}", round+1);
    // number of empty cells is the size of the area - the number of elves
    println!("result: {}", (max_y - min_y + 1)*(max_x - min_x + 1) - n_elves);
}

struct Elf {
    x: usize,
    y: usize,
}

#[derive(Clone)]
enum Proposition {
    // y, x
    Pos(usize,usize),
    None,
}

impl Elf {
    fn propose(&self, board: &Vec<Vec<bool>>, round: usize) -> Proposition {
        let mut all_empty = true;
        for i in [-1i32, 0, 1] {
            for j in [-1i32, 0 ,1] {
                if i != 0 || j != 0 {
                    if board[((self.y as i32)+i) as usize][((self.x as i32)+j) as usize] {
                        all_empty = false;
                        break;
                    }
                }
            }
        }

        if all_empty {
            return Proposition::None;
        }

        let order = round % 4;
        for i in 0..4 {
            let j = (i + order) % 4;
            match j {
                0 => {
                    let y = self.y - 1;
                    if !(board[y][self.x] || board[y][self.x-1] || board[y][self.x+1]) {
                        return Proposition::Pos(y, self.x);
                    }
                },
                1 => {
                    let y = self.y + 1;
                    if !(board[y][self.x] || board[y][self.x-1] || board[y][self.x+1]) {
                        return Proposition::Pos(y, self.x);
                    }
                },
                2 => {
                    let x = self.x - 1;
                    if !(board[self.y][x] || board[self.y-1][x] || board[self.y+1][x]) {
                        return Proposition::Pos(self.y, x);
                    }
                },
                3 => {
                    let x = self.x + 1;
                    if !(board[self.y][x] || board[self.y-1][x] || board[self.y+1][x]) {
                        return Proposition::Pos(self.y, x);
                    }
                },
                _ => {},
            }

        }
        return Proposition::None;
    }
}

fn parse_state(lines: &Vec<String>, offset: usize) -> (Vec<Vec<bool>>, Vec<Elf>, usize, usize) {
    let n = lines.len();
    let m = lines[0].len();

    let mut board = vec![vec![false; m+2*offset]; n+2*offset];
    let mut elves: Vec<Elf> = Vec::new();

    for i in 0..n {
        let line = lines[i].as_bytes();
        for j in 0..m {
            let c = line[j] as char;
            if c == '#' {
                let y = i+offset;
                let x = j + offset;
                board[y][x] = true;
                elves.push(Elf{x: x, y: y});
            }
        }
    }

    return (board, elves, n+2*offset, m+2*offset);
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
