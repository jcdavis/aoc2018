extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;

struct Line {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    content: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let re = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)").unwrap();
    let rl: Vec<Result<String>> = br.lines().collect();
    let lines: Vec<Line> = rl.iter().map(|line| {
        let l = line.as_ref().unwrap();
        let mat = re.captures(l).unwrap();
        return Line {year: mat[1].parse().unwrap(), month: mat[2].parse().unwrap(),
            day: mat[3].parse().unwrap(),
            hour: mat[4].parse().unwrap(),
            minute: mat[5].parse().unwrap(),
            content: mat[6].to_string()};
    }).collect();

    let guard_data = parse(&lines);
    part1(&guard_data);

    part2(&guard_data);
}

fn parse(lines: &Vec<Line>) -> HashMap<i32, Box<Vec<i32>>> {
    let mut guards: HashMap<i32, Box<Vec<i32>>> = HashMap::new();
    let mut current_guard = -1;
    let mut sleep_start = 0;
    let re = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    for line in lines {
        if line.content == "falls asleep" {
            sleep_start = line.minute;
        } else if line.content == "wakes up" {
            if !guards.contains_key(&current_guard) {
                let arr = Box::new(vec![0; 60]);
                guards.insert(current_guard, arr);
            };
            let curArray = guards.get_mut(&current_guard).unwrap();
            for d in sleep_start..line.minute {
                curArray[d as usize] += 1;
            }
        } else {
            let mat = re.captures(&line.content).unwrap();
            current_guard = mat[1].parse().unwrap();
        }
    }
    guards
}

fn part1(guard_data: &HashMap<i32, Box<Vec<i32>>>) -> () {
    let mut chosen_guard = -1;
    let mut total_asleep = 0;
    let mut max_min = -1;
    for (k, v) in guard_data {
        let mut this_sum = 0;
        let mut this_max_min = 0;
        let mut this_max_count = 0;
        for min in 0..60 {
            this_sum += v[min];
            if v[min] > this_max_count {
                this_max_count = v[min];
                this_max_min = min as i32;
            }
        }
        if this_sum > total_asleep {
            chosen_guard = *k;
            total_asleep = this_sum;
            max_min = this_max_min;
        }
    }
    println!("{} {}", chosen_guard, max_min);
}

fn part2(guard_data: &HashMap<i32, Box<Vec<i32>>>) -> () {
    let mut chosen_guard = -1;
    let mut max_count = 0;
    let mut max_min = -1;
    for (k, v) in guard_data {
        let mut this_max_min = 0;
        let mut this_max_count = 0;
        for min in 0..60 {
            if v[min] > this_max_count {
                this_max_count = v[min];
                this_max_min = min as i32;
            }
        }
        if this_max_count > max_count {
            chosen_guard = *k;
            max_count = this_max_count;
            max_min = this_max_min;
        }
    }
    println!("{} {}", chosen_guard, max_min);
}