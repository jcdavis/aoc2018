extern crate regex;

use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let rl: Vec<Result<String>> = br.lines().collect();
    let points: Vec<(i32,i32)> = rl.iter().map(|line| {
        let mat = re.captures(line.as_ref().unwrap()).unwrap();
        (mat[1].parse().unwrap(), mat[2].parse().unwrap())
    }).collect();

    part1(&points);
    part2(&points);
}

fn part1(points: &Vec<(i32, i32)>) {
    // 0 unknown, -1 tied
    let mut nearest: [[i16; 500]; 500] = [[0; 500]; 500];
    let mut distance: [[i16; 500]; 500] = [[0; 500]; 500];

    for i in 0..points.len() {
        let (col, row) = points[i];

        //Expand closest bfs style
        let mut queue: VecDeque<(usize, usize, i16)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        queue.push_back((row as usize, col as usize, 0));

        while !queue.is_empty() {
            let (r, c, dist) = queue.pop_front().unwrap();

            if nearest[r][c] == 0 || distance[r][c] > dist {
                nearest[r][c] = (i+1) as i16;
                distance[r][c] = dist as i16;

                if r > 0 && !visited.contains(&(r-1,c)) {
                    visited.insert((r-1, c));
                    queue.push_back((r-1, c, dist+1));
                }
                if r < 499 && !visited.contains(&(r+1,c)) {
                    visited.insert((r+1, c));
                    queue.push_back((r+1, c, dist+1));
                }
                if c > 0 && !visited.contains(&(r,c-1)) {
                    visited.insert((r, c-1));
                    queue.push_back((r, c-1, dist+1));
                }
                if c < 499 && !visited.contains(&(r,c+1)) {
                    visited.insert((r, c+1));
                    queue.push_back((r, c+1, dist+1));
                }
            } else if distance[r][c] == dist {
                nearest[r][c] = -1;
            }
        }
    }

    let mut counts: HashMap<i16, i32> = HashMap::new();
    let mut edges: HashSet<i16> = HashSet::new();

    for i in 0..500 {
        for j in 0..500 {
            let cell = nearest[i as usize][j as usize];
            if cell > 0 {
                let prev = *counts.get(&cell).unwrap_or(&0);
                counts.insert(cell, prev + 1);
            }
            if i == 0 || j == 0 || i == 499 || j == 499 {
                edges.insert(cell);
            }
        }
    }
    let mut max = 0;
    for (cell, count) in &counts {
        if !edges.contains(cell) && *count > max {
            max = *count;
        }
    }
    println!("{}", max);
}

fn part2(points: &Vec<(i32, i32)>) {
    let mut cells = 0;
    for i in -20..520 {
        for j in -20..520 {
            let mut total_dist = 0;
            for (r, c) in points {
                total_dist += (*r-i).abs() + (*c-j).abs();
            }
            if total_dist < 10000 {
                cells += 1;
            }
        }
    }
    println!("{}", cells);
}
