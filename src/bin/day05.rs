use regex::Regex;
use std::{collections::HashSet, fmt::Display, fs};

fn str2regex(inp: &str) -> String {
    // ",([0-9]*,)?"
    inp.to_owned().replace(",", ",([0-9]+,)*?") // XXX: improve regex
}

fn part1(pages: &Vec<String>, rules: &str) -> i32 {
    let mut correct_pages: Vec<String> = Vec::new();

    for (i, page) in pages.iter().enumerate() {
        let re = Regex::new(&str2regex(page)).unwrap();
        println!("{}", re.as_str());
        if re.is_match(rules) {
            println!("{i} is correct");
            correct_pages.push(page.to_owned());
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
fn part2(_inp: &str) -> usize {
    0
}

enum Insertion {
    After,
    Before,
}

fn create_rule_vec(rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut all_nums: HashSet<i32> = HashSet::new();

    for rule in rules {
        all_nums.insert(rule.0);
        all_nums.insert(rule.1);
    }
    let mut all_nums: Vec<i32> = all_nums.into_iter().collect();
    let mut sorted: Vec<i32> = Vec::new();

    println!("all_nums: {:?}", all_nums);
    sorted.push(all_nums.remove(0));

    // pick a number for which a rule exists
    //
    // enter that number directly next to the element that it pairs with
    //
    // move element in one direction according to existing rules

    // XXX: consider moving some more stuff

    let mut finished = false;
    while !finished {
        let mut loop_index = 0;
        let (index, rule): (usize, (i32, i32)) = loop {
            let el = &all_nums[loop_index];
            let mut rule = (*el, *el);
            println!("looking at {el}");
            for sl in &sorted {
                rule = match (
                    rules.iter().position(|&r| r == (*el, *sl)),
                    rules.iter().position(|&r| r == (*sl, *el)),
                ) {
                    (Some(r), None) => rules[r],
                    (None, Some(r)) => rules[r],
                    (None, None) => continue,
                    (Some(_), Some(_)) => unreachable!("SNH"),
                };
                println!("sorted: {:?}", sorted);
                println!("found rule: {:?}", rule);
                break;
            }
            if rule != (*el, *el) {
                break (loop_index, rule);
            }
            loop_index += 1;
        };

        let el = all_nums.remove(index);
        let (insertion, index) = if el == rule.1 {
            (
                Insertion::After,
                sorted.iter().position(|&p| p == rule.0).unwrap() + 1,
            )
        } else {
            // el == rule.0
            assert!(el == rule.0);
            (
                Insertion::Before,
                sorted.iter().position(|&p| p == rule.1).unwrap(),
            )
        };

        sorted.insert(index, el);
        println!("after insertion: {:?}", sorted);

        // this is insufficient, there might be rules later, that are not needed
        // to be fulfilled at this point
        match insertion {
            Insertion::After => {
                let mut max_ndx = index; // current index
                for (i, s) in sorted[index + 1..].iter().enumerate() {
                    for rule in rules {
                        if *rule == (*s, el) {
                            println!("AFT|found rule {:?}", *rule);
                            if index + 1 + i > max_ndx {
                                max_ndx = index + 1 + i;
                            }
                        }
                    }
                }
                println!("AFT|to replace: {max_ndx} with {index}");
                let moved = sorted.remove(index);
                sorted.insert(max_ndx, moved);

                // from here on out it gets crazy
                let mut movme: Vec<usize> = Vec::new();
                for (i, s) in sorted[..max_ndx].iter().enumerate() {
                    for rule in rules {
                        if *rule == (moved, *s) {
                            println!("AFT|found rule {:?}", *rule);
                            movme.push(i);
                        }
                    }
                }

                // 1 2 3 4 5 6
                // 1 2 3 5 6 4

                // 1 2 5 6 3 4

                // 2 5 6 1 3 4

                for (i, mm) in movme.iter().rev().enumerate() {
                    let tmp = sorted.remove(*mm);
                    sorted.insert(max_ndx - i, tmp);
                }
            }
            Insertion::Before => {
                let mut min_ndx = index;
                for (i, s) in sorted[..index].iter().enumerate() {
                    for rule in rules {
                        if *rule == (el, *s) {
                            println!("BEF|found rule {:?}", *rule);
                            if i < min_ndx {
                                min_ndx = i;
                            }
                        }
                    }
                }

                println!("BEF|to replace: {min_ndx} with {index}");
                let moved = sorted.remove(index);
                sorted.insert(min_ndx, moved);
            }
        }

        // for any rule, check if there is a pair with the current element and any following/preceding

        if all_nums.len() == 0 {
            finished = true;
        }
    }

    println!("sorted {:?}", sorted);
    sorted
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

fn vec2string<T>(inp: &Vec<T>) -> String
where
    T: Display,
{
    let mut out = String::new();
    for v in inp[..inp.len() - 1].iter() {
        out.push_str(&format!("{},", v));
    }
    out.push_str(&format!("{}", inp[inp.len() - 1]));
    out
}

fn main() {
    println!("Day 5");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let (rules, pages) = extract_input(&data);
    let rule_vec = create_rule_vec(&rules);
    let rule_string = vec2string(&rule_vec);
    println!("rule_string: {rule_string}");

    // println!("{:?}", rules);
    // println!("{:?}", pages);

    let total_p1 = part1(&pages, &rule_string);
    println!("{total_p1}");
    // let total_p2 = part2(&data);
    // assert!(total_p1 == 143); // real input part 1
    assert!(total_p1 == 5762); // real input part 1

    let total_p2 = part2(&data);
    println!("{total_p2}");
    assert!(total_p2 == 1796);
}
