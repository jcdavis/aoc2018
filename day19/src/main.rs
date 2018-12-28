extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

type Reg = [i32; 6];

fn addr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] + before[ops[2] as usize];
    res
}

fn addi(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] + ops[2];
    res
}

fn mulr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] * before[ops[2] as usize];
    res
}

fn muli(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] * ops[2];
    res
}

fn banr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] & before[ops[2] as usize];
    res
}

fn bani(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] & ops[2];
    res
}

fn borr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] | before[ops[2] as usize];
    res
}

fn bori(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize] | ops[2];
    res
}

fn setr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = before[ops[1] as usize];
    res
}

fn seti(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = ops[1];
    res
}

fn gtir(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = if ops[1]  > before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn gtri(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] > ops[2] { 1 } else { 0 };
    res
}

fn gtrr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] > before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn eqir(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = if ops[1]  == before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn eqri(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] == ops[2] { 1 } else { 0 };
    res
}

fn eqrr(before: Reg, ops: [i32; 4]) -> Reg {
    let mut res = before.clone();
    res[ops[3] as usize] = if before[ops[1] as usize] == before[ops[2] as usize] { 1 } else { 0 };
    res
}

fn lines_from_file(name: &str) -> Vec<String> {
    let f = File::open(name).unwrap();
    let br = BufReader::new(f);

    br.lines().map(|x| x.unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = lines_from_file(&args[1]);
    let mut iter = lines.iter();
    let re = Regex::new(r"(\w+) (\d+) (\d+) (\d+)").unwrap();

    let ip = iter.next().unwrap().split_at(4).1.parse::<usize>().unwrap();
    let lines: Vec<(String, [i32; 4])> = iter.map(|line| {
        let mat = re.captures(line).unwrap();
        let bytes = [0, mat[2].parse().unwrap(), mat[3].parse().unwrap(), mat[4].parse().unwrap()];
        (mat[1].to_string(), bytes)
    }).collect();

    println!("#ip {}", ip);
    let mut reg: Reg = [0; 6];

    loop {
        let inst = reg[ip];
        if inst < 0 || inst >= lines.len() as i32 {
            break;
        }
        let (op_name, op_bytes) = &lines[inst as usize];

        reg = match op_name.as_ref() {
            "addr" => addr(reg, *op_bytes),
            "addi" => addi(reg, *op_bytes),
            "mulr" => mulr(reg, *op_bytes),
            "muli" => muli(reg, *op_bytes),
            "banr" => banr(reg, *op_bytes),
            "bani" => bani(reg, *op_bytes),
            "borr" => borr(reg, *op_bytes),
            "bori" => bori(reg, *op_bytes),
            "setr" => setr(reg, *op_bytes),
            "seti" => seti(reg, *op_bytes),
            "gtir" => gtir(reg, *op_bytes),
            "gtri" => gtri(reg, *op_bytes),
            "gtrr" => gtrr(reg, *op_bytes),
            "eqir" => eqir(reg, *op_bytes),
            "eqri" => eqri(reg, *op_bytes),
            "eqrr" => eqrr(reg, *op_bytes),
            _ => unimplemented!()
        };

        reg[ip] += 1;
    }
    println!("{}", reg[0]);
}
