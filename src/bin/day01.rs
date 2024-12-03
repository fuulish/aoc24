use std::fs;

fn main() {
    println!("Day 1");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let _ = data
        .lines()
        .map(|s| {
            let mut itr = s.split_whitespace();
            left.push(itr.next().unwrap().parse().unwrap());
            right.push(itr.next().unwrap().parse().unwrap());
        })
        .count();

    left.sort();
    right.sort();

    assert!(left.len() == right.len());
    let mut right_iter = right.iter();

    let total: i32 = left
        .iter()
        .map(|x| (x - right_iter.next().unwrap()).abs())
        .sum();
    println!("total: {total}");
    assert!(total == 1189304);

    let mut found = std::collections::HashMap::<i32, i32>::new();
    let leftover = right.iter();

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
    assert!(similarity == 24349736);
}
