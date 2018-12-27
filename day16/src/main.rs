extern crate regex;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use regex::{Captures, Regex};

struct TestCase {
    before: [i32; 4],
    after: [i32; 4],
    opcodes: [i32; 4],
}

type Op = Fn([i32; 4], [i32; 4]) -> [i32; 4];

fn addr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] + before[ops[2] as usize];
    res
}

fn addi(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] + ops[2];
    res
}

fn mulr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] * before[ops[2] as usize];
    res
}

fn muli(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] * ops[2];
    res
}

fn banr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] & before[ops[2] as usize];
    res
}

fn bani(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] & ops[2];
    res
}

fn borr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] | before[ops[2] as usize];
    res
}

fn bori(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] | ops[2];
    res
}

fn setr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize];
    res
}

fn seti(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = ops[1];
    res
}

fn gtir(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = if ops[1]  > before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn gtri(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] > ops[2] { 1 } else { 0 };
    res
}

fn gtrr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] > before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn eqir(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = if ops[1]  == before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn eqri(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] == ops[2] { 1 } else { 0 };
    res
}

fn eqrr(before: [i32; 4], ops: [i32; 4]) -> [i32; 4] {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] == before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn match_to_arr(mat: &Captures) -> [i32; 4] {
    [mat[1].parse().unwrap(), mat[2].parse().unwrap(), mat[3].parse().unwrap(), mat[4].parse().unwrap()]
}

fn lines_from_file(name: &str) -> Vec<String> {
    let f = File::open(name).unwrap();
    let br = BufReader::new(f);

    br.lines().map(|x| x.unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let state_re = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let op_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let lines = lines_from_file(&args[1]);
    let mut iter = lines.iter().peekable();
    let mut cases: Vec<TestCase> = Vec::new();

    while iter.peek().is_some() {
        let before_str = iter.next().unwrap();
        let before = state_re.captures(&before_str).unwrap();
        let ops_str = iter.next().unwrap();
        let ops = op_re.captures(&ops_str).unwrap();
        let after_str = iter.next().unwrap();
        let after = state_re.captures(&after_str).unwrap();
        cases.push(TestCase { before: match_to_arr(&before), after: match_to_arr(&after), opcodes: match_to_arr(&ops) });
        //Eat empty line
        iter.next();
    }

    let ops: Vec<&Op> = vec![&addr, &addi, &mulr, &muli, &banr, &bani, &borr, &bori, &setr, &seti, &gtir, &gtri, &gtrr, &eqir, &eqri, &eqrr];

    let mut candidates: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 0..ops.len() {
        candidates.insert(i, HashSet::from_iter(0..ops.len()));
    }

    for case in &cases {
        for i in 0..ops.len() {
            if ops[i](case.before, case.opcodes) != case.after {
                candidates.get_mut(&(case.opcodes[0] as usize)).map(|set| set.remove(&i));
            }
        }
    }

    let mut mappings: HashMap<usize, usize> = HashMap::new();

    loop {
        let mut found = 1000;

        for (op, ids) in &candidates {
            if ids.len() == 1 {
                found = *op;
                break;
            }
        }
        if found == 1000 {
            break;
        }
        let from = *candidates.get(&found).unwrap().iter().next().unwrap();
        mappings.insert(found, from);

        for (_, ids) in candidates.iter_mut() {
            ids.remove(&from);
        }
    }
    let command_lines = lines_from_file(&args[2]);

    let mut state = [0; 4];
    for line in &command_lines {
        let op_bytes = match_to_arr(&op_re.captures(line).unwrap());

        state = ops[*mappings.get(&(op_bytes[0] as usize)).unwrap()](state, op_bytes);
    }
    println!("{}", state[0]);
}
