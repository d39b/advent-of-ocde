use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    let lines = read_file("input.txt");
    
    // // part 1
    // // can still be easily "brute forced"
    // let (monkeys, root_monkey_index, _) = parse_monkeys(&lines);
    // let n = monkeys.len();

    // let mut values = vec![0; n];
    // let mut evaluated = vec![false; n];
    // let mut c = 0;
    // while c < n {
    //     for i in 0..n {
    //         if !evaluated[i] {
    //             match &monkeys[i] {
    //                 Monkey::Num(x) => {
    //                     values[i] = *x;
    //                     evaluated[i] = true;
    //                     c += 1;
    //                 },
    //                 Monkey::Wait(i1, i2, op) => {
    //                     if evaluated[*i1] && evaluated[*i2] {
    //                         values[i] = match op {
    //                             Op::Add => values[*i1] + values[*i2],
    //                             Op::Sub => values[*i1] - values[*i2],
    //                             Op::Mult => values[*i1] * values[*i2],
    //                             Op::Div => values[*i1] / values[*i2],
    //                         };
    //                         evaluated[i] = true;
    //                         c += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("result: {}", values[root_monkey_index]);

    // part 2
    // represent the computation as a tree and reverse it
    // this works easy because the "humn" variable only appears once
    let (monkeys, root_monkey_index, human_index) = parse_monkeys(&lines);

    let root = build_tree(root_monkey_index, &monkeys, root_monkey_index, human_index);
    let eval_tree = eval_tree(&root);
    println!("{}", tree_to_string(&eval_tree));
    let h_value = prune_tree(&eval_tree, 0);
    println!("result: {}", h_value);
    println!("check: {}", eval_tree_with_human(&eval_tree, h_value));
}

#[derive(Clone)]
enum Node {
    Human,
    Num(i64),
    Equal(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

fn tree_to_string(node: &Node) -> String {
    match node {
        Node::Human => return "H".to_string(),
        Node::Num(x) => (*x).to_string(),
        Node::Equal(a, b) => {
            let s1 = tree_to_string(a);
            let s2 = tree_to_string(b);
            let mut result = String::new();
            result.push_str(&s1);
            result.push_str(" = ");
            result.push_str(&s2);
            return result;
        }
        Node::Add(a, b) => {
            let s1 = tree_to_string(a);
            let s2 = tree_to_string(b);
            let mut result = String::new();
            result.push_str("(");
            result.push_str(&s1);
            result.push_str(" + ");
            result.push_str(&s2);
            result.push_str(")");
            return result;
        }
        Node::Sub(a, b) => {
            let s1 = tree_to_string(a);
            let s2 = tree_to_string(b);
            let mut result = String::new();
            result.push_str("(");
            result.push_str(&s1);
            result.push_str(" - ");
            result.push_str(&s2);
            result.push_str(")");
            return result;
        }
        Node::Mult(a, b) => {
            let s1 = tree_to_string(a);
            let s2 = tree_to_string(b);
            let mut result = String::new();
            result.push_str("(");
            result.push_str(&s1);
            result.push_str(" * ");
            result.push_str(&s2);
            result.push_str(")");
            return result;
        }
        Node::Div(a, b) => {
            let s1 = tree_to_string(a);
            let s2 = tree_to_string(b);
            let mut result = String::new();
            result.push_str("(");
            result.push_str(&s1);
            result.push_str(" / ");
            result.push_str(&s2);
            result.push_str(")");
            return result;
        }
    }
}

fn prune_tree(node: &Node, curr: i64) -> i64 {
    match node {
        Node::Human => curr,
        Node::Num(x) => *x,
        Node::Equal(a, b) => {
            match **a {
                Node::Num(x) => {
                    return prune_tree(b, x);
                },
                _ => {},
            };
            match **b {
                Node::Num(x) => {
                    return prune_tree(a, x);
                },
                _ => panic!(),
            };
        }, 
        Node::Add(a, b) => {
            match **a {
                Node::Num(x) => {
                    return prune_tree(b, curr-x);
                },
                _ => {},
            };
            match **b {
                Node::Num(x) => {
                    return prune_tree(a, curr-x);
                },
                _ => panic!(),
            };
        },
        Node::Sub(a, b) => {
            match **a {
                Node::Num(x) => {
                    return prune_tree(b, x-curr);
                },
                _ => {},
            };
            match **b {
                Node::Num(x) => {
                    return prune_tree(a, curr+x);
                },
                _ => panic!(),
            };
        },
        Node::Mult(a, b) => {
            match **a {
                Node::Num(x) => {
                    return prune_tree(b, curr/x);
                },
                _ => {},
            };
            match **b {
                Node::Num(x) => {
                    return prune_tree(a, curr/x);
                },
                _ => panic!(),
            };
        },
        Node::Div(a, b) => {
            match **a {
                Node::Num(x) => {
                    return prune_tree(b, x/curr);
                },
                _ => {},
            };
            match **b {
                Node::Num(x) => {
                    return prune_tree(a, curr*x);
                },
                _ => panic!(),
            };
        },
    }
}

fn eval_tree(node: &Node) -> Node {
    return match node {
        Node::Human => Node::Human,
        Node::Num(x) => Node::Num(*x),
        Node::Equal(a, b) => Node::Equal(
            Box::new(eval_tree(a)),
            Box::new(eval_tree(b)),
        ),
        Node::Add(a, b) => {
            let n1 = eval_tree(a);
            let n2 = eval_tree(b);
            match n1 {
                Node::Num(x) => match n2 {
                    Node::Num(y) => {
                        return Node::Num(x+y);
                    },
                    _ => {},
                }
                _ => {},
            }
            return Node::Add(Box::new(n1), Box::new(n2));
        },
        Node::Sub(a, b) => {
            let n1 = eval_tree(a);
            let n2 = eval_tree(b);
            match n1 {
                Node::Num(x) => match n2 {
                    Node::Num(y) => {
                        return Node::Num(x-y);
                    },
                    _ => {},
                }
                _ => {},
            }
            return Node::Sub(Box::new(n1), Box::new(n2));
        },
        Node::Mult(a, b) => {
            let n1 = eval_tree(a);
            let n2 = eval_tree(b);
            match n1 {
                Node::Num(x) => match n2 {
                    Node::Num(y) => {
                        return Node::Num(x*y);
                    },
                    _ => {},
                }
                _ => {},
            }
            return Node::Mult(Box::new(n1), Box::new(n2));
        },
        Node::Div(a, b) => {
            let n1 = eval_tree(a);
            let n2 = eval_tree(b);
            match n1 {
                Node::Num(x) => match n2 {
                    Node::Num(y) => {
                        return Node::Num(x/y);
                    },
                    _ => {},
                }
                _ => {},
            }
            return Node::Div(Box::new(n1), Box::new(n2));
        },
    };
}

fn eval_tree_with_human(node: &Node, h: i64) -> i64 {
    return match node {
        Node::Human => h,
        Node::Num(x) => *x,
        Node::Equal(a, b) => eval_tree_with_human(a, h) - eval_tree_with_human(b, h),
        Node::Add(a, b) => {
            eval_tree_with_human(a, h) + eval_tree_with_human(b, h)
        },
        Node::Sub(a, b) => {
            eval_tree_with_human(a, h) - eval_tree_with_human(b, h)
        },
        Node::Mult(a, b) => {
            eval_tree_with_human(a, h) * eval_tree_with_human(b, h)
        },
        Node::Div(a, b) => {
            eval_tree_with_human(a, h) / eval_tree_with_human(b, h)
        },
    };
}

fn build_tree(i: usize, monkeys: &Vec<Monkey>, root: usize, human: usize) -> Node {
    if i == human {
        return Node::Human;
    }
    if i == root {
        let (i1, i2) = match &monkeys[i] {
            Monkey::Wait(x, y, _) => (*x, *y),
            _ => panic!(),
        };
        return Node::Equal(
            Box::new(build_tree(i1, monkeys, root, human)),
            Box::new(build_tree(i2, monkeys, root, human)),
        );
    } else {
        return match &monkeys[i] {
            Monkey::Num(x) => Node::Num(*x),
            Monkey::Wait(x, y, op) => match *op {
                Op::Add => Node::Add(
                    Box::new(build_tree(*x, monkeys, root, human)),
                    Box::new(build_tree(*y, monkeys, root, human)),
                ),
                Op::Sub => Node::Sub(
                    Box::new(build_tree(*x, monkeys, root, human)),
                    Box::new(build_tree(*y, monkeys, root, human)),
                ),
                Op::Mult => Node::Mult(
                    Box::new(build_tree(*x, monkeys, root, human)),
                    Box::new(build_tree(*y, monkeys, root, human)),
                ),
                Op::Div => Node::Div(
                    Box::new(build_tree(*x, monkeys, root, human)),
                    Box::new(build_tree(*y, monkeys, root, human)),
                ),
            },
        }
    }
}

enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

enum Monkey {
    Num(i64),
    Wait(usize, usize, Op)
}

fn parse_monkeys(s: &Vec<String>) -> (Vec<Monkey>, usize, usize) {
    let mut result = Vec::new();

    let mut name_to_index: HashMap<&str, usize> = HashMap::new();
    let mut root_monkey_index = 0;
    let mut human_index = 0;
    for i in 0..s.len() {
        let name = s[i].split(':').next().unwrap();
        name_to_index.insert(name, i);
        if name == "root" {
            root_monkey_index = i;
        }
        if name == "humn" {
            human_index = i;
        }
    }

    for i in 0..s.len() {
        let parts = s[i].split(" ").collect::<Vec<&str>>();
        if parts.len() > 2 {
            result.push(Monkey::Wait(
                *name_to_index.get(parts[1]).unwrap(),
                *name_to_index.get(parts[3]).unwrap(),
                parse_op(parts[2]),
            ));
        } else {
            result.push(Monkey::Num(parts[1].parse().unwrap()));
        }
    }

    return (result, root_monkey_index, human_index);
}

fn parse_op(s: &str) -> Op {
    match s {
        "+" => Op::Add,
        "-" => Op::Sub,
        "/" => Op::Div,
        "*" => Op::Mult,
        _ => panic!(),
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