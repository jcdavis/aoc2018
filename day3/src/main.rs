extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let lines: Vec<Result<String>> = br.lines().collect();
    let mut arr: [[u16; 2000]; 2000] = [[0; 2000]; 2000];
    let re = Regex::new(r"#\d+\s+@ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in &lines {
        let l = line.as_ref().unwrap();
        let mat = re.captures(l).unwrap();
        let col: i32 = mat[1].parse().unwrap();
        let row: i32 = mat[2].parse().unwrap();
        let width: i32 = mat[3].parse().unwrap();
        let height: i32 = mat[4].parse().unwrap();
        //println!("{} {} {} {}", row, col, width, height);
        for i in row..row+height {
            for j in col..col+width {
                arr[i as usize][j as usize] += 1;
            }
        }
    }
    let mut overlaps = 0;

    for row in arr.iter() {
        for cell in row.iter() {
            if *cell >= 2 {
                overlaps += 1;
            }
        }
    }
    println!("{}", overlaps);
}