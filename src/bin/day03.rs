use std::fs;

fn main() {
    println!("Day 3");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let re = regex::Regex::new(r"(mul\(([0-9]+),([0-9]+)\))").unwrap();

    let mut total = 0;
    for c in re.captures_iter(&data) {
        let a: i32 = c.get(2).unwrap().as_str().parse().unwrap();
        let b: i32 = c.get(3).unwrap().as_str().parse().unwrap();
        total += a * b;
        /*
        println!("{}", c.get(1).unwrap().as_str());
        println!("{}", c.get(3).unwrap().as_str());
        println!("{}", c.get(4).unwrap().as_str());
        */
    }

    println!("{total}");
    // assert!(total == 161); // test input part 1
    assert!(total == 153469856); // real input part 1
}
