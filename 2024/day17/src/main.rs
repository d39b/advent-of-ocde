use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // let (registers, program) = read_input("example.txt");
    // let (registers, program) = read_input("example2.txt");
    let (registers, program) = read_input("input.txt");
    part1(&registers, &program);
    part2(&program);
}

fn part1(registers: &Vec<i64>, program: &Vec<u8>) {
    let mut a = registers[0];
    let mut b = registers[1];
    let mut c = registers[2];

    let mut ip = 0;
    let mut out: Vec<i64> = vec![];

    let combo_val = |v: u8, a: i64, b: i64, c: i64| -> i64 {
        if v <= 3 {
            return v as i64;
        } else if v == 4 {
            return a;
        } else if v == 5 {
            return b;
        } else if v == 6 {
            return c;
        } else {
            panic!();
        }
    };

    while ip + 1 < program.len() {
        let ins = program[ip];
        let operand = program[ip + 1];
        match ins {
            0 => {
                let denom = 1 << combo_val(operand, a, b, c);
                a /= denom;
                ip += 2;
            }
            1 => {
                b = b ^ (operand as i64);
                ip += 2;
            }
            2 => {
                b = combo_val(operand, a, b, c) % 8;
                ip += 2;
            }
            3 => {
                if a == 0 {
                    ip += 2;
                } else {
                    ip = operand as usize;
                }
            }
            4 => {
                b = b ^ c;
                ip += 2;
            }
            5 => {
                out.push(combo_val(operand, a, b, c) % 8);
                ip += 2;
            }
            6 => {
                let denom = 1 << combo_val(operand, a, b, c);
                b = a / denom;
                ip += 2;
            }
            7 => {
                let denom = 1 << combo_val(operand, a, b, c);
                c = a / denom;
                ip += 2;
            }
            _ => {}
        }
    }

    print!("{}", out[0]);
    for i in 1..out.len() {
        print!(",{}", out[i]);
    }
    println!();
}

fn part2(program: &Vec<u8>) {
    // after some investigating we find that the program runs the following operations:
    // B = A % 8
    // B = B ^ 3
    // C = A / (1 << B)
    // A = A / 8
    // B = B ^ 4
    // B = B ^ C
    // output B % 8
    // repeat
    // for the output at each step only the lower 3 bits of B matter
    // initially they are the lower 3 bits of A and then A is shifted 3 bits to the right
    // i.e. in every iteration we consider the next 3 bits of A
    // we do B xor 3 and later B xor 4 which boils down to B xor 7 which just flips all the lower 3
    // bits in B
    // i.e. B is the lower 3 bits of A flipped
    // and then we xor with C which A / (1 << B) and b is the lower 3 bits of A xor 3
    // at most B could be 7
    // so C = A / (1 << 7) i.e. it would remove up to 7 bits from A and then xor the next 3 with b
    // so the output depends on at most the lower 10 bits of A
    // for each of these possibilities we can compute the digit that is output
    // then through backtracking we can try to stitch these 10 digits blocks together to form a bit
    // sequence that outputs the correct number
    let n = 1 << 10;
    let mut output = vec![vec![]; 8];
    for x in 0..n {
        let mut b = x % 8;
        b = b ^ 3;
        // let c = x / (1 << b);
        let c = x >> b;
        b = b ^ 4;
        b = b ^ c;
        output[b % 8].push(x);
    }

    let ex: [usize; 16] = [2, 4, 1, 3, 7, 5, 0, 3, 1, 4, 4, 7, 5, 5, 3, 0];
    let expected: Vec<u8> = vec![2, 4, 1, 3, 7, 5, 0, 3, 1, 4, 4, 7, 5, 5, 3, 0];
    let z = backtrack(&ex, &output, 16, 0);
    println!("{z}");
    println!("{}", sim(z, program, &expected));
}

const MASK: usize = (1 << 10) - 1;

fn backtrack(ex: &[usize; 16], output: &Vec<Vec<usize>>, i: usize, v: usize) -> usize {
    if i == 0 {
        return v;
    }

    if i == ex.len() {
        for &z in output[ex[i - 1]].iter() {
            let y = backtrack(ex, output, i - 1, z);
            if y != 0 {
                return y;
            }
        }
    } else {
        let x = v << 3;
        for &z in output[ex[i - 1]].iter() {
            if ((x | (z & 7)) & MASK) == z {
                let y = backtrack(ex, output, i - 1, x | z);
                if y != 0 {
                    return y;
                }
            }
        }
    }

    return 0;
}

fn sim(a: usize, program: &Vec<u8>, expected: &Vec<u8>) -> bool {
    let c1 = sim1(a as i64, program, expected);
    let c2 = sim2(a, expected);
    if c1 != c2 {
        println!("short: {c2}  long: {c1}");
        panic!("simulation values are different");
    }
    return c1;
}

fn sim2(a: usize, expected: &Vec<u8>) -> bool {
    let mut a = a;
    for &d in expected {
        let mut b = a % 8;
        b = b ^ 3;
        let c = a >> b;
        b = b ^ 4;
        b = b ^ c;
        a = a / 8;
        if (b % 8) != (d as usize) {
            return false;
        }
    }
    return true;
}

fn sim1(a: i64, program: &Vec<u8>, expected: &Vec<u8>) -> bool {
    let mut a = a;
    let mut b = 0;
    let mut c = 0;

    let mut ip = 0;
    let mut out_i = 0;

    let combo_val = |v: u8, a: i64, b: i64, c: i64| -> i64 {
        if v <= 3 {
            return v as i64;
        } else if v == 4 {
            return a;
        } else if v == 5 {
            return b;
        } else if v == 6 {
            return c;
        } else {
            panic!();
        }
    };

    while ip + 1 < program.len() {
        let ins = program[ip];
        let operand = program[ip + 1];
        match ins {
            0 => {
                let denom = 1 << combo_val(operand, a, b, c);
                a /= denom;
                ip += 2;
            }
            1 => {
                b = b ^ (operand as i64);
                ip += 2;
            }
            2 => {
                b = combo_val(operand, a, b, c) % 8;
                ip += 2;
            }
            3 => {
                if a == 0 {
                    ip += 2;
                } else {
                    ip = operand as usize;
                }
            }
            4 => {
                b = b ^ c;
                ip += 2;
            }
            5 => {
                let z = combo_val(operand, a, b, c) % 8;
                if (expected[out_i] as i64) != z {
                    return false;
                }
                out_i += 1;
                if out_i >= expected.len() {
                    return true;
                }
                ip += 2;
            }
            6 => {
                let denom = 1 << combo_val(operand, a, b, c);
                b = a / denom;
                ip += 2;
            }
            7 => {
                let denom = 1 << combo_val(operand, a, b, c);
                c = a / denom;
                ip += 2;
            }
            _ => {}
        }
    }
    if out_i < expected.len() {
        return false;
    }
    return true;
}

fn read_input(f: &str) -> (Vec<i64>, Vec<u8>) {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut registers = vec![];
    for i in 0..3 {
        let (_, r) = lines[i].split_once(':').unwrap();
        registers.push(r.trim().parse::<i64>().unwrap());
    }
    let (_, r) = lines[4].split_once(':').unwrap();
    let program = r
        .trim()
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    return (registers, program);
}
