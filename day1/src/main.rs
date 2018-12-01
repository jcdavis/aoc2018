use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let mut sum = 0;
    for line in br.lines() {
      let l = line.unwrap();
      let delta = l[1..].parse::<i32>().unwrap();
      if l.chars().next().unwrap() == '+' {
        sum += delta;
      } else {
        sum -= delta;
      }
    }
    println!("Sum: {}", sum);
}
