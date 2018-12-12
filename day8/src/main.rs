use std::env;
use std::fs::File;
use std::io::prelude::*;

struct TreeNode {
    children: Vec<Box<TreeNode>>,
    metadata: Vec<i32>,
}

fn part1(node: &TreeNode) -> i32 {
    let mut total = 0;
    for child in &node.children {
        total += part1(child);
    }
    for meta in &node.metadata {
        total += *meta;
    }
    total
}

fn part2(node: &TreeNode) -> i32 {
    if node.children.is_empty() {
        node.metadata.iter().sum()
    } else {
        let mut total = 0;
        for i in &node.metadata {
            let index = *i-1;
            if index >= 0 && index < node.children.len() as i32  {
                total += part2(&node.children[index as usize]);
            }
        }
        total
    }
}

fn parse_node(nums: &Vec<i32>, pos: usize) -> Option<(TreeNode, usize)> {
    if nums.len() == pos {
        return None;
    }
    let mut mpos = pos;
    let num_children = nums[mpos];
    let num_metadata = nums[mpos+1];
    mpos += 2;

    let mut children: Vec<Box<TreeNode>> = Vec::new();

    for _ in 0..num_children {
        let (child, next_pos) = parse_node(nums, mpos).unwrap();
        mpos = next_pos;
        children.push(Box::new(child));
    }

    let mut metadata: Vec<i32> = Vec::new();

    for _ in 0..num_metadata {
        metadata.push(nums[mpos]);
        mpos += 1;
    }
    Some((TreeNode {
        children: children,
        metadata: metadata,
    }, mpos))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).expect("???");
    let nums: Vec<i32> = data.trim().split(" ").map(|n| n.parse().unwrap()).collect();

    let (parsed, _) = parse_node(&nums, 0).unwrap();
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}
