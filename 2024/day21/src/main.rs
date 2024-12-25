use std::collections::HashMap;

fn main() {
    let codes = vec!["140A", "170A", "169A", "803A", "129A"];
    part1(&codes, 2);
    part1(&codes, 25);
}

// iterative solution, actually similar in runtime to the recursive one
fn part1(codes: &Vec<&str>, robots: usize) {
    let key_to_pos = HashMap::from([
        (b'7', (0, 0)),
        (b'8', (0, 1)),
        (b'9', (0, 2)),
        (b'4', (1, 0)),
        (b'5', (1, 1)),
        (b'6', (1, 2)),
        (b'1', (2, 0)),
        (b'2', (2, 1)),
        (b'3', (2, 2)),
        (b'0', (3, 1)),
        (b'A', (3, 2)),
    ]);

    let n = 2 * 3;
    // i = r*3 + c
    // d^k(i, j) = min number of key presses to move finger of k-th robot in sequence from
    // position i to j on the keypad
    // every iteration adds another robot on top
    let mut d_prev = vec![vec![1; n]; n];
    for _ in 0..robots {
        let mut d_curr = vec![vec![0; n]; n];
        for i in 1..n {
            let (r1, c1) = (i / 3, i % 3);
            for j in 1..n {
                let (r2, c2) = (j / 3, j % 3);
                let mut min = 1 << 62;
                for t in gen_key_sequence(r1, c1, r2, c2, 0, 0) {
                    let mut v = 0;
                    let mut curr_r = 0;
                    let mut curr_c = 2;
                    for (r, c) in t {
                        let i1 = curr_r * 3 + curr_c;
                        let i2 = r * 3 + c;
                        v += d_prev[i1][i2];
                        curr_r = r;
                        curr_c = c;
                    }

                    if v < min {
                        min = v;
                    }
                }
                d_curr[i][j] = min;
            }
        }
        d_prev = d_curr;
    }

    let m = 4 * 3;
    // i = r*3 + c;
    // gap is on (3, 0) -> i = 9
    let mut d = vec![vec![0; m]; m];
    for i in 0..m {
        if i == 9 {
            continue;
        }
        let (r1, c1) = (i / 3, i % 3);
        for j in 0..m {
            if j == 9 {
                continue;
            }
            let (r2, c2) = (j / 3, j % 3);
            let mut min = 1 << 62;
            for t in gen_key_sequence(r1, c1, r2, c2, 3, 0) {
                let mut v = 0;
                let mut curr_r = 0;
                let mut curr_c = 2;
                for (r, c) in t {
                    let i1 = curr_r * 3 + curr_c;
                    let i2 = r * 3 + c;
                    v += d_prev[i1][i2];
                    curr_r = r;
                    curr_c = c;
                }

                if v < min {
                    min = v;
                }
            }
            d[i][j] = min;
        }
    }

    let mut result = 0;
    for &c in codes {
        let cv = c.trim_matches('A').parse::<usize>().unwrap();
        let x = c.as_bytes();
        let mut target = vec![];
        for b in x.iter() {
            let &(r, c) = key_to_pos.get(b).unwrap();
            target.push((r, c));
        }
        let mut curr_r = 3;
        let mut curr_c = 2;
        let mut v = 0;
        for (next_r, next_c) in target {
            let i1 = curr_r * 3 + curr_c;
            let i2 = next_r * 3 + next_c;
            v += d[i1][i2];
            curr_r = next_r;
            curr_c = next_c;
        }
        result += v * cv;
    }

    println!("{result}");
}

fn gen_key_sequence(
    sr: usize,
    sc: usize,
    er: usize,
    ec: usize,
    fr: usize,
    fc: usize,
) -> Vec<Vec<(usize, usize)>> {
    // positive means move to right, negative means move to left
    let horz = (ec as i32) - (sc as i32);
    // positive means move down, negative means move up
    let vert = (er as i32) - (sr as i32);

    if horz == 0 && vert == 0 {
        return vec![vec![(0, 2)]];
    } else {
        // move horz first
        let mut result = vec![];
        if !(fc == ec && fr == sr) {
            let mut seq = vec![];
            add_horz(&mut seq, horz);
            add_vert(&mut seq, vert);
            seq.push((0, 2));
            result.push(seq);
        }
        // move vert first
        if !(fr == er && fc == sc) {
            let mut seq = vec![];
            add_vert(&mut seq, vert);
            add_horz(&mut seq, horz);
            seq.push((0, 2));
            result.push(seq);
        }
        return result;
    }
}

fn add_horz(seq: &mut Vec<(usize, usize)>, v: i32) {
    if v >= 0 {
        for _ in 0..v {
            seq.push((1, 2));
        }
    } else {
        for _ in 0..-v {
            seq.push((1, 0));
        }
    }
}

fn add_vert(seq: &mut Vec<(usize, usize)>, v: i32) {
    if v >= 0 {
        for _ in 0..v {
            seq.push((1, 1));
        }
    } else {
        for _ in 0..-v {
            seq.push((0, 1));
        }
    }
}
