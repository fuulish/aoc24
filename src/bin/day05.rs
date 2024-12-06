use regex::Regex;
use std::fs;

fn part1(pages: &Vec<String>, rules: &Vec<(i32, i32)>) -> i32 {
    let mut correct_pages: Vec<String> = Vec::new();

    for (i, page) in pages.iter().enumerate() {
        let mut incorrect = false;
        println!("working on page {}", i);
        for rule in rules {
            let re = Regex::new(&format!(r"{}.*{}", rule.1, rule.0)).unwrap();

            if let Some(_) = re.find(page) {
                incorrect = true;
            }
        }
        if !incorrect {
            correct_pages.push(page.to_owned())
        }
    }

    let mut total = 0;
    for page in correct_pages {
        let num: Vec<i32> = page.split(",").map(|v| v.parse::<i32>().unwrap()).collect();

        // println!("{:?} {}", num, num[num.len() / 2]);

        total += num[num.len() / 2];
    }
    total
}
fn part2(inp: &str) -> usize {
    0
}

fn extract_input(inp: &str) -> (Vec<(i32, i32)>, Vec<String>) {
    let mut rules = Vec::new();
    let mut pages = Vec::new();

    let mut get_rule = true;

    for line in inp.lines() {
        if get_rule {
            if line.len() == 0 {
                get_rule = false;
                continue;
            }

            let mut itr = line.split("|");
            // probably don't need the parse
            let first = itr.next().unwrap().parse::<i32>().unwrap();
            let second = itr.next().unwrap().parse::<i32>().unwrap();
            rules.push((first, second));
        } else {
            pages.push(line.to_owned());
        }
    }

    (rules, pages)
}

fn main() {
    println!("Day 5");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let (rules, pages) = extract_input(&data);

    // println!("{:?}", rules);
    // println!("{:?}", pages);

    let total_p1 = part1(&pages, &rules);
    println!("{total_p1}");
    // let total_p2 = part2(&data);
    assert!(total_p1 == 143); // real input part 1

    let total_p2 = part2(&data);
    println!("{total_p2}");
    assert!(total_p2 == 1796);
}
