use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let mut sum = 0;
    let lines: Vec<Result<String>> = br.lines().collect();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut shouldContinue = true;
    while shouldContinue {
      for line in &lines {
        if seen.contains(&sum) {
          shouldContinue = false;
          break;
        }
        seen.insert(sum);
        let l = line.as_ref().unwrap();
        let delta = l[1..].parse::<i32>().unwrap();
        if l.chars().next().unwrap() == '+' {
          sum += delta;
        } else {
          sum -= delta;
        }
      }
    }
    println!("Sum: {}", sum);
}
