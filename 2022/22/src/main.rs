use core::panic;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;
use std::cmp::{min, max};

fn main() {
    let lines = read_file("input.txt");

    let (board, n, m) = parse_board(&lines);

    let moves = parse_moves(&lines);

    // let n = 4;
    let n = 50;
    // let side_boards = vec![
    //     extract_slice(8, 0, n, &board),
    //     extract_slice(0, 4, n, &board),
    //     extract_slice(4, 4, n, &board),
    //     extract_slice(8, 4, n, &board),
    //     extract_slice(8, 8, n, &board),
    //     extract_slice(12, 8, n, &board),
    // ];
    let side_boards = vec![
        extract_slice(50, 0, n, &board),
        extract_slice(100, 0, n, &board),
        extract_slice(50, 50, n, &board),
        extract_slice(0, 100, n, &board),
        extract_slice(50, 100, n, &board),
        extract_slice(0, 150, n, &board),
    ];

    // let sides = vec![
    //     Side{
    //         right: (5, Dir::Right),
    //         left: (2, Dir::Up),
    //         up: (1, Dir::Up),
    //         down: (3, Dir::Up),
    //     },
    //     Side{
    //         right: (2, Dir::Left),
    //         left: (5, Dir::Down),
    //         up: (0, Dir::Up),
    //         down: (4, Dir::Down),
    //     },
    //     Side{
    //         right: (3, Dir::Left),
    //         left: (1, Dir::Right),
    //         up: (0, Dir::Left),
    //         down: (4, Dir::Left),
    //     },
    //     Side{
    //         right: (5, Dir::Up),
    //         left: (2, Dir::Right),
    //         up: (0, Dir::Down),
    //         down: (4, Dir::Up),
    //     },
    //     Side{
    //         right: (5, Dir::Left),
    //         left: (2, Dir::Down),
    //         up: (3, Dir::Down),
    //         down: (1, Dir::Down),
    //     },
    //     Side{
    //         right: (0, Dir::Right),
    //         left: (4, Dir::Right),
    //         up: (3, Dir::Right),
    //         down: (1, Dir::Left),
    //     },
    // ];

    let sides = vec![
        Side{
            right: (1, Dir::Left),
            left: (3, Dir::Left),
            up: (5, Dir::Left),
            down: (2, Dir::Up),
        },
        Side{
            right: (4, Dir::Right),
            left: (0, Dir::Right),
            up: (5, Dir::Down),
            down: (2, Dir::Right),
        },
        Side{
            right: (1, Dir::Down),
            left: (3, Dir::Up),
            up: (0, Dir::Down),
            down: (4, Dir::Up),
        },
        Side{
            right: (4, Dir::Left),
            left: (0, Dir::Left),
            up: (2, Dir::Left),
            down: (5, Dir::Up),
        },
        Side{
            right: (1, Dir::Right),
            left: (3, Dir::Right),
            up: (2, Dir::Down),
            down: (5, Dir::Right),
        },
        Side{
            right: (4, Dir::Down),
            left: (0, Dir::Up),
            up: (3, Dir::Down),
            down: (1, Dir::Up),
        },
    ];

    let mut player = Player {
        board: &side_boards,
        sides: &sides,
        n: n,
        side: 0,
        x: 0,
        y: 0,
        dir: Dir::Right,
    };
    
    for m in moves.iter() {
        player.m(m);
        println!("pos: {}, {}, {}", player.side, player.x, player.y);
    }

    println!("final pos: {}, {}, {}", player.side, player.x, player.y);
    println!("result: {}", (player.x+1+50)*4 + (player.y+1+50)*1000 + (player.dir as usize));
}

fn extract_slice(x: usize, y: usize, n: usize, old: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result = vec![vec![Tile::Empty; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = old[x+i][y+j];
        }
    }

    return result;
}

struct Player<'a> {
    board: &'a Vec<Vec<Vec<Tile>>>,
    sides: &'a Vec<Side>,
    side: usize,
    n: usize,
    x: usize,
    y: usize,
    dir: Dir,
}

// maybe this can be done easier if we define for each pair of sides that share an edge
// what edge it is of each side, e.g. right and left
//  and based on that it should be easier to calculate the next step

struct Side {
    right: (usize, Dir),
    left: (usize, Dir),
    up: (usize, Dir),
    down: (usize, Dir),
}

impl Player<'_> {
    fn m(&mut self, m: &Move) {
        match *m {
            Move::S(x) => self.m_steps(x),
            Move::TRight => self.m_turn(true),
            Move::TLeft => self.m_turn(false),
        }
    }

    fn inv_x(&self) -> usize {
        return self.n-1-self.x;
    }

    fn inv_y(&self) -> usize {
        return self.n-1-self.y;
    }

    fn m_steps(&mut self, c: u64) {
        for i in 0..c {
            let (side, x, y, d) = self.m_step();
            if self.board[side][x][y] == Tile::Wall {
                break;
            } else {
                self.side = side;
                self.x = x;
                self.y = y;
                self.dir = d;
            }
        }
    }

    fn m_step(&mut self) -> (usize, usize, usize, Dir) {
        match self.dir {
            Dir::Right => {
                if self.x < self.n - 1 {
                    return (self.side, self.x+1, self.y, self.dir);
                }
                let (si, d) = self.sides[self.side].right;
                let mut nextDir = Dir::Right;
                let mut nextX = 0;
                let mut nextY = 0;
                match d {
                    Dir::Right => {
                        nextX = self.n-1;
                        nextY = self.inv_y();
                        nextDir = Dir::Left;
                    },
                    Dir::Left => {
                        nextX = 0;
                        nextY = self.y;
                        nextDir = Dir::Right;
                    },
                    Dir::Up => {
                        nextX = self.inv_y();
                        nextY = 0;
                        nextDir = Dir::Down;
                    },
                    Dir::Down => {
                        nextX = self.y;
                        nextY = self.n-1;
                        nextDir = Dir::Up;
                    },
                }
                return (si, nextX, nextY, nextDir);
            },
            Dir::Left => {
                if self.x > 0 {
                    return (self.side, self.x-1, self.y, self.dir);
                }
                let (si, d) = self.sides[self.side].left;
                let mut nextDir = Dir::Left;
                let mut nextX = 0;
                let mut nextY = 0;
                match d {
                    Dir::Right => {
                        nextX = self.n-1;
                        nextY = self.y;
                        nextDir = Dir::Left;
                    },
                    Dir::Left => {
                        nextX = 0;
                        nextY = self.inv_y();
                        nextDir = Dir::Right;
                    },
                    Dir::Up => {
                        nextX = self.y;
                        nextY = 0;
                        nextDir = Dir::Down;
                    },
                    Dir::Down => {
                        nextX = self.inv_y();
                        nextY = self.n-1;
                        nextDir = Dir::Up;
                    },
                }
                return (si, nextX, nextY, nextDir);
            },
            Dir::Up => {
                if self.y > 0 {
                    return (self.side, self.x, self.y-1, self.dir);
                }
                let (si, d) = self.sides[self.side].up;
                let mut nextDir = Dir::Up;
                let mut nextX = 0;
                let mut nextY = 0;
                match d {
                    Dir::Right => {
                        nextX = self.n-1;
                        nextY = self.inv_x();
                        nextDir = Dir::Left;
                    },
                    Dir::Left => {
                        nextX = 0;
                        nextY = self.x;
                        nextDir = Dir::Right;
                    },
                    Dir::Up => {
                        nextX = self.inv_x();
                        nextY = 0;
                        nextDir = Dir::Down;
                    },
                    Dir::Down => {
                        nextX = self.x;
                        nextY = self.n-1;
                        nextDir = Dir::Up;
                    },
                }
                return (si, nextX, nextY, nextDir);
            },
            Dir::Down => {
                if self.y < self.n-1 {
                    return (self.side, self.x, self.y+1, self.dir);
                }
                let (si, d) = self.sides[self.side].down;
                let mut nextDir = Dir::Down;
                let mut nextX = 0;
                let mut nextY = 0;
                match d {
                    Dir::Right => {
                        nextX = self.n-1;
                        nextY = self.x;
                        nextDir = Dir::Left;
                    },
                    Dir::Left => {
                        nextX = 0;
                        nextY = self.inv_x();
                        nextDir = Dir::Right;
                    },
                    Dir::Up => {
                        nextX = self.x;
                        nextY = 0;
                        nextDir = Dir::Down;
                    },
                    Dir::Down => {
                        nextX = self.inv_x();
                        nextY = self.n-1;
                        nextDir = Dir::Up;
                    },
                }
                return (si, nextX, nextY, nextDir);
            },
        }
    }

    fn m_turn(&mut self, right: bool) {
        if right {
            self.dir = match self.dir {
                Dir::Right => Dir::Down,
                Dir::Left => Dir::Up,
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
            }
        } else {
            self.dir = match self.dir {
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
            }
        }
    }

}


#[derive(Debug,Clone,Copy,PartialEq)]
enum Tile {
    Empty,
    Open,
    Wall,
}

#[derive(Debug,Clone,Copy,PartialEq)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug,Clone,Copy)]
enum Move {
    S(u64),
    TRight,
    TLeft,
}

fn parse_board(lines: &Vec<String>) -> (Vec<Vec<Tile>>, usize, usize) {
    let n = lines.len()-2;
    let mut m = 0;
    for s in lines.iter() {
        if s.len() > m {
            m = s.len();
        }
    } 

    let mut board = vec![vec![Tile::Empty; n]; m];

    for i in 0..n {
        let b = lines[i].as_bytes();
        for j in 0..b.len() {
            let c = b[j] as char;
            if c == '.' {
                board[j][i] = Tile::Open;
            } else if c == '#' {
                board[j][i] = Tile::Wall;
            }
        }
    }

    return (board, n, m);
}

fn parse_moves(lines: &Vec<String>) -> Vec<Move> {
    let line = lines.last().unwrap();

    let mut result = Vec::new();

    let mut nums = Vec::new();
    for num in line.split(['R', 'L']) {
        nums.push(Move::S(num.parse().unwrap()));
    }

    let mut turns = Vec::new();
    for x in line.as_bytes().iter() {
        let c = (*x) as char;
        if c == 'R' {
            turns.push(Move::TRight);
        } else if c == 'L' {
            turns.push(Move::TLeft);
        }
    }

    for i in 0..turns.len() {
        result.push(nums[i]);
        result.push(turns[i]);
    }
    result.push(*nums.last().unwrap());

    return result;
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
