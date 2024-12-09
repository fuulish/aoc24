use std::cmp::Ordering;
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
            rules.insert((first, second), Ordering::Less);
            rules.insert((second, first), Ordering::Greater);
        } else {
            pages.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }

    (rules, pages)
}

type RuleSet = HashMap<(i32, i32), Ordering>;

enum PageType {
    Correct,
    Incorrect,
}

fn get_pages(rules: &RuleSet, pages: &Vec<Vec<i32>>, correctness: PageType) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();

    for page in pages {
        let mut is_correct = true;
        for (lindex, &left) in page.iter().enumerate() {
            'outer: for &right in page[lindex + 1..].iter() {
                if *rules.get(&(left, right)).unwrap_or(&Ordering::Less) != Ordering::Less {
                    is_correct = false;
                    break 'outer;
                }
            }
        }

        output.push(match correctness {
            PageType::Correct => {
                if is_correct {
                    page.clone()
                } else {
                    continue;
                }
            }
            PageType::Incorrect => {
                if !is_correct {
                    page.clone()
                } else {
                    continue;
                }
            }
        })
    }

    output
}

fn part1(rules: &RuleSet, pages: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    let correct_pages = get_pages(rules, pages, PageType::Correct);

    for page in correct_pages {
        total += page[page.len() / 2];
    }

    total
}

fn part2(rules: &RuleSet, pages: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    let incorrect_pages = get_pages(rules, pages, PageType::Incorrect);

    for page in incorrect_pages {
        let mut count = Vec::new();
        for &right in &page {
            let idx = count.len();
            count.push((right, 0));
            for &left in &page {
                if *rules.get(&(left, right)).unwrap_or(&Ordering::Greater) == Ordering::Greater {
                    count[idx].1 += 1;
                }
            }
        }
        count.sort_by_key(|&x| x.1);
        total += count[count.len() / 2].0;
    }

    total
}

fn part2_alt(rules: &RuleSet, pages: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    let incorrect_pages = get_pages(rules, pages, PageType::Incorrect);

    for page in incorrect_pages {
        let mut sorted = page.clone();
        sorted.sort_by(|a, b| match rules.get(&(*a, *b)) {
            Some(&v) => v,
            None => {
                println!("rule missing"); // <- runtime says none is missing
                Ordering::Less
            }
        });
        total += sorted[sorted.len() / 2];
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
    let total_p2_alt = part2_alt(&rules, &pages);
    println!("{total_p2}");
    assert!(total_p2 == total_p2_alt);
    assert!(total_p2 == 4130);
}
