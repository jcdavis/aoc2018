use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).expect("???");

    println!("{}", get_len(&data));

    let mut min: usize = 10000;
    for letter in 65..91 {
        let filtered: String = data.chars().filter(|c| {
            let num = *c as u8;
            num != letter && num != letter+32
        }).collect();
        let len = get_len(&filtered);
        if len < min {
            min = len;
        }
    }
    println!("{}", min);
}

fn get_len(string: &String) -> usize {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        let complement = if c >= 'a' { (c as u8) - 32 } else { (c as u8) + 32 };
        if stack.last() == Some(&(complement as char)) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    //Off by one, presumably trailing \n?
    stack.len() - 1
}
