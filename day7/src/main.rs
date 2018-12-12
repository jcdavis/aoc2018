extern crate regex;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::option::Option;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);

    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let rl: Vec<Result<String>> = br.lines().collect();
    let edges: Vec<(String, String)> = rl.iter().map(|line| {
        let mat = re.captures(line.as_ref().unwrap()).unwrap();
        (mat[1].to_string(), mat[2].to_string())
    }).collect();

    let mut outbound: HashMap<&String, HashSet<&String>> = HashMap::new();
    let mut inbound: HashMap<&String, HashSet<&String>> = HashMap::new();

    for (source, dest) in &edges {
        if outbound.contains_key(source) {
            outbound.get_mut(source).unwrap().insert(dest);
        } else {
            let mut set: HashSet<&String> = HashSet::new();
            set.insert(dest);
            outbound.insert(source, set);
        }
        if inbound.contains_key(dest) {
            inbound.get_mut(dest).unwrap().insert(source);
        } else {
            let mut set: HashSet<&String> = HashSet::new();
            set.insert(source);
            inbound.insert(dest, set);
        }
    }
    let mut no_inc: BTreeSet<String> = BTreeSet::new();

    for i in b'A'..b'Z' {
        let ch = i as char;
        let as_str = ch.to_string();
        if !inbound.contains_key(&as_str) {
            no_inc.insert(as_str);
        }
    }

    part1(&outbound, &mut inbound.clone(), &mut no_inc.clone());
    part2(&outbound, &mut inbound, &mut no_inc, 5);
}

fn remove_lowest(no_inc: &mut BTreeSet<String>) -> Option<String> {
    let low_opt = no_inc.iter().next().map(|t| (*t).clone());
    low_opt.map(|s| {
        no_inc.remove(&s);
        s
    })
}
fn remove_from_inbound(src: &String, dest: &String, inbound: &mut HashMap<&String, HashSet<&String>>) -> bool {
    let ibs = inbound.get_mut(dest).unwrap();
    ibs.remove(src);
    return ibs.is_empty();
}

fn part1(outbound: &HashMap<&String, HashSet<&String>>, inbound: &mut HashMap<&String, HashSet<&String>>, no_inc: &mut BTreeSet<String>) {
    while !no_inc.is_empty() {
        let elem: String = remove_lowest(no_inc).unwrap();
        no_inc.remove(&elem);
        print!("{}", elem);
        for dest in outbound.get(&elem).unwrap_or(&HashSet::new()) {
            if remove_from_inbound(&elem, *dest, inbound) {
                no_inc.insert((**dest).clone());
            }
        }
    }
    println!("");
}

fn part2(outbound: &HashMap<&String, HashSet<&String>>, inbound: &mut HashMap<&String, HashSet<&String>>, no_inc: &mut BTreeSet<String>, workers: usize) {
    let mut processing: BTreeMap<i32, BTreeSet<String>> = BTreeMap::new();
    let mut last_time = 0;
    loop {
        while processing.len() < workers {
            match remove_lowest(no_inc) {
                Some(s) => {
                    let time = (s.chars().next().unwrap() as i32) - 4;
                    processing.entry(last_time + time).or_insert_with(|| BTreeSet::new()).insert(s);
                }
                None => break
            };
        }
        let (new_time, s, should_remove) = {
            let t = processing.iter_mut().next();
            if t.is_none() {
                println!("{}", last_time);
                return;
            }
            let (nt, set) = t.unwrap();
            let s = remove_lowest(set).unwrap();
            (*nt, s, set.is_empty())
        };
        if should_remove {
            processing.remove(&new_time);
        }
        last_time = new_time;
        for dest in outbound.get(&s).unwrap_or(&HashSet::new()) {
            if remove_from_inbound(&s, *dest, inbound) {
                no_inc.insert((**dest).clone());
            }
        }
    }
}
