use std::{collections::HashMap, fs};

fn extract_input(inp: &str) -> (RuleSet, Vec<Vec<i32>>) {
    let mut rules: RuleSet = HashMap::new();
    let mut get_rule = true;
    let mut pages: Vec<Vec<i32>> = Vec::new();

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
            rules.insert((first, second), true);
            rules.insert((second, first), false);
        } else {
            pages.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }

    (rules, pages)
}

type RuleSet = HashMap<(i32, i32), bool>;

fn get_incorrect_pages(rules: &RuleSet, pages: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut incorrect_pages: Vec<Vec<i32>> = Vec::new();

    for page in pages {
        let mut is_correct = true;
        for (lindex, &left) in page.iter().enumerate() {
            'outer: for &right in page[lindex + 1..].iter() {
                if !rules.get(&(left, right)).unwrap_or(&true) {
                    is_correct = false;
                    break 'outer;
                }
            }
        }
        if !is_correct {
            incorrect_pages.push(page.clone());
        }
    }

    incorrect_pages
}

fn part1(rules: &RuleSet, pages: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    let mut correct_pages: Vec<Vec<i32>> = Vec::new();

    for page in pages {
        let mut is_correct = true;
        for (lindex, &left) in page.iter().enumerate() {
            'outer: for &right in page[lindex + 1..].iter() {
                if !rules.get(&(left, right)).unwrap_or(&true) {
                    is_correct = false;
                    break 'outer;
                }
            }
        }
        if is_correct {
            correct_pages.push(page.clone());
        }
    }

    for page in correct_pages {
        total += page[page.len() / 2];
    }

    total
}

fn part2(rules: &RuleSet, pages: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    let incorrect_pages = get_incorrect_pages(rules, pages);

    for page in incorrect_pages {
        let mut count = Vec::new();
        for &left in &page {
            let idx = count.len();
            count.push((left, 0));
            for &right in &page {
                if *rules.get(&(right, left)).unwrap_or(&false) {
                    count[idx].1 += 1;
                }
            }
        }
        count.sort_by_key(|&x| x.1);
        total += count[count.len() / 2].0;
    }

    total
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

    let total_p1 = part1(&rules, &pages);
    println!("{total_p1}");
    // let total_p2 = part2(&data);
    // assert!(total_p1 == 143); // real input part 1
    // assert!(total_p1 == 5762); // real input part 1

    let total_p2 = part2(&rules, &pages);
    println!("{total_p2}");
    assert!(total_p2 == 4130);
}
