extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;

struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

fn dist(a: &Nanobot, b: &Nanobot) -> i32 {
    (a.x-b.x).abs() + (a.y-b.y).abs() + (a.z-b.z).abs()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let rl: Vec<Result<String>> = br.lines().collect();
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();

    let mut bots: Vec<Nanobot> = rl.iter().map(|line| {
        let mat = re.captures(line.as_ref().unwrap()).unwrap();
        Nanobot { x: mat[1].parse().unwrap(), y: mat[2].parse().unwrap(), z: mat[3].parse().unwrap(), r: mat[4].parse().unwrap() }
    }).collect();

    bots.sort_unstable_by_key(|bot| bot.r);

    let bot = bots.last().unwrap();

    let mut in_range = 0;
    for other in &bots {
        if dist(bot, other) <= bot.r {
            in_range += 1;
        }
    }
    println!("{}", in_range);
}
