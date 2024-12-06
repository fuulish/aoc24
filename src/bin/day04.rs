use std::fs;

fn into_diagonal_lines(inp: &str) -> String {}

fn into_vertical_lines(inp: &str) -> String {
    for line in inp.lines() {}
}

fn part1(inp: &str) -> usize {
    let count_xmas = {
        |line: &str| {
            let mut start: usize = 0;
            let mut count: usize = 0;
            while let Some(index) = line[start..].find("XMAS") {
                start += index + 4;
                count += 1;
            }
            count
        }
    };

    let mut total = inp.lines().map(|line| count_xmas(line)).sum();
    total += inp
        .lines()
        // XXX: potentially extract into sub-function
        .map(|line| line.chars().rev().collect::<String>())
        .map(|rev| count_xmas(&rev))
        .sum::<usize>();

    // there's a number of ways to flatten the array
    //      - lines concatenated horizontally (and reversed)
    //      - lines concatenated vertically (and reversed)
    //      - lines concatenated diagonally (and reversed)
    //
    // however, we cannot simply concatenate them,
    // because, that would imply line wrap around

    total
}

fn part2(inp: &str) -> i32 {
    0
}

fn main() {
    println!("Day 4");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let total_p1 = part1(&data);
    println!("{total_p1}");
    // let total_p2 = part2(&data);
    assert!(total_p1 == 18); // real input part 1
}
