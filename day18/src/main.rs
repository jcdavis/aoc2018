use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let rl: Vec<String> = br.lines().map(|x| x.as_ref().unwrap().clone()).collect();

    let mut current: Vec<Vec<char>> = Vec::new();
    let mut next: Vec<Vec<char>> = Vec::new();

    for line in &rl {
        let as_chars: Vec<char> = line.chars().collect();
        current.push(as_chars.clone());
        next.push(as_chars);
    }

    for i in 1..1000000000 {
        iteration(&current, &mut next);
        mem::swap(&mut current, &mut next);
        
        let mut tree_count = 0;
        let mut yard_count = 0;

        for row in &current {
            for cell in row {
                if *cell == '|' {
                    tree_count += 1;
                }
                if *cell == '#' {
                    yard_count += 1;
                }
            }
        }
        let mut s = DefaultHasher::new();
        current.hash(&mut s);
        println!("{}: {} {}", i, tree_count*yard_count, s.finish());
    }
}

fn iteration(current: &Vec<Vec<char>>, next: &mut Vec<Vec<char>>) {
    for row in 0..50 {
        for col in 0..50 {
            let (_, tree, yard) = neighbors(current, row as i32, col as i32);
            let ch = current[row][col];
            next[row][col] = ch;
            if ch == '.' && tree >= 3 {
                next[row][col] = '|';
            }
            if ch == '|' && yard >= 3 {
                next[row][col] = '#';
            }
            if ch == '#' && !(yard >= 1 && tree >= 1) {
                next[row][col] = '.';
            }
        }
    }
}

fn neighbors(current: &Vec<Vec<char>>, row: i32, col: i32) -> (i32, i32, i32) {
    let mut open = 0;
    let mut tree = 0;
    let mut yard = 0;

    for r in (row-1)..(row+2) {
        for c in (col-1)..(col+2) {
            if r >= 0 && r < 50 && c >= 0 && c < 50 && (r != row || c != col) {
                let ch = current[r as usize][c as usize];
                if ch == '.' {
                    open += 1;
                }
                if ch == '|' {
                    tree += 1;
                }
                if ch == '#' {
                    yard += 1;
                }
            }
        }
    }
    (open, tree, yard)
}
