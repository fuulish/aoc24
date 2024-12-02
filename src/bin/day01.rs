use aoc24::add;
use std::fs;

fn main() {
    println!("Day 1 {}", add(30, 12));
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let _ = data.lines().map(|s| {
        let mut i = s.split(" ");
        left.push(i.next().unwrap().parse().unwrap());
        right.push(i.next().unwrap().parse().unwrap());
    });
}
