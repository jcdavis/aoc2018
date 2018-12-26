extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let state_re = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let op_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let mut iter = br.lines().map(|x| x.unwrap()).peekable();
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

    let mut three_or_more = 0;
    for case in &cases {
        let mut count = 0;
        for op in &ops {
            if op(case.before, case.opcodes) == case.after {
                count += 1;
            }
        }
        if count >= 3 {
            three_or_more += 1;
        }
    }
    println!("{}", three_or_more);
}
