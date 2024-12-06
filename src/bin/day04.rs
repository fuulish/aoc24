use std::fs;

fn into_left_diagonal_lines(inp: &str) -> String {
    let mut diag = String::new();
    let lines: Vec<&str> = inp.lines().collect();
    let dim = lines.len();

    for i in 0..dim {
        for j in 0..i + 1 {
            // this is the upper half triangle
            diag.push(lines[i - j].as_bytes()[j] as char);
        }
        diag.push('\n');
    }
    // full diagonal already covered in the above
    for i in 1..dim {
        for j in 0..dim - i {
            diag.push(lines[dim - 1 - j].as_bytes()[j + i] as char)
        }
        diag.push('\n');
    }

    diag
}

fn into_right_diagonal_lines(inp: &str) -> String {
    let mut diag = String::new();
    let lines: Vec<&str> = inp.lines().collect();
    let dim = lines.len();

    for i in 0..dim {
        for j in 0..i + 1 {
            // this is the upper half triangle
            diag.push(lines[j].as_bytes()[dim - 1 + j - i] as char);
        }
        diag.push('\n');
    }
    // full diagonal already covered in the above
    for i in 1..dim {
        for j in 0..dim - i {
            diag.push(lines[i + j].as_bytes()[j] as char)
        }
        diag.push('\n');
    }

    diag
}

fn into_vertical_lines(inp: &str) -> String {
    let mut verti = String::new();
    let lines: Vec<&str> = inp.lines().collect();
    let dim = lines.len();

    for i in 0..dim {
        for j in 0..dim {
            verti.push(lines[j].as_bytes()[i] as char)
        }
        verti.push('\n');
    }

    verti
}

fn count_xmas(inp: &str) -> usize {
    let xmas_per_string = {
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

    let mut total = inp.lines().map(|line| xmas_per_string(line)).sum::<usize>();
    total += inp
        .lines()
        // XXX: potentially extract into sub-function
        // XXX: or just search for the reverse string... "SAMX"
        .map(|line| line.chars().rev().collect::<String>())
        .map(|rev| xmas_per_string(&rev))
        .sum::<usize>();

    total
}

fn part1(inp: &str) -> usize {
    let mut total: usize = count_xmas(inp);

    // there's a number of ways to flatten the array
    //      - lines concatenated horizontally (and reversed)
    //      - lines concatenated vertically (and reversed)
    //      - lines concatenated diagonally (and reversed)
    //
    // however, we cannot simply concatenate them,
    // because, that would imply line wrap around

    total += count_xmas(&into_vertical_lines(inp));
    total += count_xmas(&into_left_diagonal_lines(inp));
    total += count_xmas(&into_right_diagonal_lines(inp));
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
    assert!(total_p1 == 2378); // real input part 1
}
