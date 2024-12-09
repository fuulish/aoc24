use std::fs;

const TEST_SOLUTION_PART1: i32 = 3749;
const TEST_SOLUTION_PART2: i32 = 0;

const KNOWN_SOLUTION_PART1: i32 = 41;
const KNOWN_SOLUTION_PART2: i32 = 0;

fn part1() -> i32 {
    0
}
fn part2() -> i32 {
    0
}

fn main() {
    println!("Day 6");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let use_test = std::env::var("TEST").is_ok();

    let total_p1 = part1();
    println!("{total_p1}");
    if use_test {
        assert!(total_p1 == TEST_SOLUTION_PART1);
    } else {
        assert!(total_p1 == KNOWN_SOLUTION_PART1);
    }

    let total_p2 = part2();
    println!("{total_p2}");
    if use_test {
        assert!(total_p2 == TEST_SOLUTION_PART2);
    } else {
        assert!(total_p2 == KNOWN_SOLUTION_PART2);
    }
}
