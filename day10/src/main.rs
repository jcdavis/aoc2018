extern crate regex;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);

    let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let rl: Vec<Result<String>> = br.lines().collect();
    let edges: Vec<((i32, i32), (i32 ,i32))> = rl.iter().map(|line| {
        let mat = re.captures(line.as_ref().unwrap()).unwrap();
        ((mat[1].parse().unwrap(), mat[2].parse().unwrap()), (mat[3].parse().unwrap(), mat[4].parse().unwrap()))
    }).collect();

    let mut positions: Vec<(i32, i32)> = edges.iter().map(|(p,_)| p.clone()).collect();
    let vectors: Vec<(i32, i32)> = edges.iter().map(|(_,v)| v.clone()).collect();

    let mut iteration = 0;
    loop {
        for i in 0..positions.len() {
            let (x,y) = positions[i];
            let (dx, dy) = vectors[i];
            positions[i] = (x+dx, y+dy);
        }

        let p_set: HashSet<(i32, i32)> = positions.iter().map(|t| t.clone()).collect();

        let mut nearby_count = 0;

        for (x,y) in &positions {
            let mut has_nearby = false;
            for dx in -1..2 {
                for dy in -1..2 {
                    if (dx != 0 || dy != 0) && p_set.contains(&(*x+dx, *y+dy)) {
                        has_nearby = true;
                    }
                }
            }
            if has_nearby {
                nearby_count += 1;
            }
        }
        iteration += 1;
        if nearby_count > 300 {
            let pos_x: Vec<i32> = positions.iter().map(|(x,_)| *x).collect();
            let pos_y: Vec<i32> = positions.iter().map(|(_,y)| *y).collect();
            let min_x = *pos_x.iter().min().unwrap();
            let max_x = *pos_x.iter().max().unwrap();
            let min_y = *pos_y.iter().min().unwrap();
            let max_y = *pos_y.iter().max().unwrap();

            for y in min_y..(max_y+1) {
                for x in min_x..(max_x+1) {
                    if p_set.contains(&(x,y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }

            println!("Iteration {}", iteration);
            return;
        }
    }
}
