use regex;
use std::fs;

fn main() {
    println!("Day 1");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let re = regex::Regex::new(r"(?<left>[0-9]+)\s+(?<right>[0-9]+)").unwrap();

    let _ = data
        .lines()
        .map(|s| {
            // println!("{s}");
            let Some(parts) = re.captures(s) else {
                panic!("aaaaaah");
            };
            let left_num = &parts["left"];
            let right_num = &parts["right"];

            left.push(left_num.parse::<i32>().unwrap());
            right.push(right_num.parse::<i32>().unwrap());
        })
        .count();

    left.sort();
    right.sort();

    // println!("{:?}", left);
    // println!("{:?}", right);

    assert!(left.len() == right.len());
    let mut right_iter = right.iter();

    let total: i32 = left
        .iter()
        .map(|x| (x - right_iter.next().unwrap()).abs())
        .sum();
    println!("total: {total}");

    let mut found = std::collections::HashMap::<i32, i32>::new();
    let leftover = right.iter();

    // if in hashmap, add corresponding value, if not, iterate over left-over list
    let similarity: i32 = left
        .iter()
        .map(|x| match found.get(&x) {
            Some(v) => *v,
            None => {
                let start = leftover.len();
                let leftover: Vec<i32> = leftover.clone().filter(|v| *v != x).copied().collect();
                let v = ((start - leftover.len()) as i32) * *x;
                found.insert(*x, v);
                v
            }
        })
        .sum();

    println!("similarity: {similarity}");
}
