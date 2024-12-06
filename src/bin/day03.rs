use std::fs;

fn part1(inp: &str) -> i32 {
    let re = regex::Regex::new(r"(mul\(([0-9]+),([0-9]+)\))").unwrap();

    let mut total = 0;
    for c in re.captures_iter(inp) {
        let a: i32 = c.get(2).unwrap().as_str().parse().unwrap();
        let b: i32 = c.get(3).unwrap().as_str().parse().unwrap();
        total += a * b;
    }

    total
}

fn part2(inp: &str) -> i32 {
    let dat = "do()".to_owned() + inp + "don't()";
    let re = regex::Regex::new(r"(do\(\)(?s:.*?)don't\(\))").unwrap();

    let mut total = 0;
    for c in re.captures_iter(&dat) {
        match c.get(1) {
            Some(v) => {
                total += part1(v.as_str());
            }
            None => (),
        }
    }
    total
}

fn main() {
    println!("Day 3");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let total_p1 = part1(&data);
    println!("{total_p1}");
    // assert!(total_p1 == 161); // test input part 1
    let total_p2 = part2(&data);
    // assert!(total_p2 == 48);
    println!("{total_p2}");
    assert!(total_p2 == 77055967); // real input part 1
}
