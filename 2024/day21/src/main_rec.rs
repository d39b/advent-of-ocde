use std::collections::HashMap;

fn main() {
    let codes = vec!["140A", "170A", "169A", "803A", "129A"];
    part1(&codes, 2);
    part1(&codes, 25);
}

// this was really hard, memoization was the key to part 2
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
    let mut result = 0;
    for &c in codes {
        let cv = c.trim_matches('A').parse::<usize>().unwrap();
        let x = c.as_bytes();
        let mut target = vec![];
        for b in x.iter() {
            let &(r, c) = key_to_pos.get(b).unwrap();
            target.push((r, c));
        }
        let mut mem = HashMap::new();
        let v = rec_keypad(&target, robots + 1, &mut mem);
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

fn rec_keypad(
    target: &[(usize, usize)],
    robots: usize,
    mem: &mut HashMap<(usize, usize, usize, usize, usize), usize>,
) -> usize {
    if robots == 0 {
        return target.len();
    }

    let mut result = 0;
    let (mut curr_r, mut curr_c) = (3, 2);
    for i in 0..target.len() {
        let (next_r, next_c) = target[i];
        let mut z = 1 << 60;
        for t in gen_key_sequence(curr_r, curr_c, next_r, next_c, 3, 0) {
            let v = rec_remote(&t, robots - 1, mem);
            if v < z {
                z = v;
            }
        }
        result += z;
        curr_r = next_r;
        curr_c = next_c;
    }
    return result;
}

fn rec_remote(
    target: &Vec<(usize, usize)>,
    robots: usize,
    mem: &mut HashMap<(usize, usize, usize, usize, usize), usize>,
) -> usize {
    if robots == 0 {
        return target.len();
    }

    let mut result = 0;
    let (mut curr_r, mut curr_c) = (0, 2);
    for i in 0..target.len() {
        let (next_r, next_c) = target[i];
        result += rec_2(curr_r, curr_c, next_r, next_c, robots, mem);
        curr_r = next_r;
        curr_c = next_c;
    }
    return result;
}

fn rec_2(
    curr_r: usize,
    curr_c: usize,
    next_r: usize,
    next_c: usize,
    robots: usize,
    mem: &mut HashMap<(usize, usize, usize, usize, usize), usize>,
) -> usize {
    if let Some(v) = mem.get(&(curr_r, curr_c, next_r, next_c, robots)) {
        return *v;
    }

    let mut result = 1 << 62;
    for t in gen_key_sequence(curr_r, curr_c, next_r, next_c, 0, 0) {
        let v = rec_remote(&t, robots - 1, mem);
        if v < result {
            result = v;
        }
    }
    mem.insert((curr_r, curr_c, next_r, next_c, robots), result);
    return result;
}
