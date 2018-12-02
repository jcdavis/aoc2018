use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let br = BufReader::new(f);
    let lines: Vec<Result<String>> = br.lines().collect();
    /*let mut has_two = 0;
    let mut has_three = 0;*/
    let line_len = lines[0].as_ref().unwrap().chars().count();
    for line in &lines {
        /*let mut counts: [i32; 256] = [0; 256];
        let l = line.as_ref().unwrap();
        for character in l.chars() {
          counts[character as usize]+= 1;
        }
        let mut this_two = false;
        let mut this_three = false;
        for count in counts.iter() {
          if *count == 2 {
            this_two = true;
          }
          if *count == 3 {
            this_three = true
          }
        }
        if this_two {
          has_two+=1;
        }
        if this_three {
          has_three+=1;
        }*/
      let l = line.as_ref().unwrap();
      for other in &lines {
        let o = other.as_ref().unwrap();
        let mut same =0;
        for t in l.chars().zip(o.chars()) {
          let (left, right) = t;
          if left == right {
            same += 1;
          }
        }
        if same + 1 == line_len {
          println!("{} {}", l, o);
          return;
        } 
      }
    }
    //println!("Sum: {}", has_two*has_three);
}
