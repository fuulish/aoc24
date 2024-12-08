use std::{collections::HashMap, fs};

const TEST_SOLUTION_PART1: i32 = 41;
const TEST_SOLUTION_PART2: i32 = 0;

const KNOWN_SOLUTION_PART1: i32 = 5312;
const KNOWN_SOLUTION_PART2: i32 = 0;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(map: &Map, starting_pos: &Pos) -> i32 {
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut direction = Direction::Up;
    let xdim = map[0].len() as i32;
    let ydim = map.len() as i32;

    let mut pos = *starting_pos;

    loop {
        // println!("current position: {:?}", pos);
        // XXX: there must be an easier way of doing this...
        match visited.get_mut(&(pos.0, pos.1)) {
            Some(v) => {
                *v += 1;
            }
            None => {
                visited.insert((pos.0, pos.1), 1);
                ()
            }
        }

        let mut next = match direction {
            Direction::Up => (pos.0, pos.1 - 1) as Pos,
            Direction::Down => (pos.0, pos.1 + 1) as Pos,
            Direction::Left => (pos.0 - 1, pos.1) as Pos,
            Direction::Right => (pos.0 + 1, pos.1) as Pos,
        };

        if next.0 < 0 || next.0 >= xdim || next.1 < 0 || next.1 >= ydim {
            break;
        }

        // println!("{}", map[next.1 as usize][next.0 as usize]);
        // println!("{:?} {} {}", map, next.1, next.0);
        (direction, next) = if map[next.1 as usize][next.0 as usize] {
            match direction {
                Direction::Up => (Direction::Right, (pos.0 + 1, pos.1) as Pos),
                Direction::Down => (Direction::Left, (pos.0 - 1, pos.1) as Pos),
                Direction::Left => (Direction::Up, (pos.0, pos.1 - 1) as Pos),
                Direction::Right => (Direction::Down, (pos.0, pos.1 + 1) as Pos),
            }
        } else {
            (direction, next)
        };

        if next.0 < 0 || next.0 >= xdim || next.1 < 0 || next.1 >= ydim {
            break;
        }

        pos = next;
    }

    // println!("{:?}", visited);
    visited.len() as i32
}
fn part2() -> i32 {
    0
}

type Map = Vec<Vec<bool>>;
type Pos = (i32, i32);

fn extract_input(data: &str) -> (Map, Pos) {
    let mut map = Map::new();
    let mut pos = (0, 0);
    for (i, line) in data.lines().enumerate() {
        let horizontal: Vec<bool> = line.chars().into_iter().map(|c| c == '#').collect();
        // println!("{:?}", horizontal);
        map.push(horizontal);

        if let Some(p) = line.chars().into_iter().position(|p| p == '^') {
            pos = (p as i32, i as i32);
        }
    }

    (map, pos)
}

fn main() {
    println!("Day 6");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let (map, pos) = extract_input(&data);

    let use_test = std::env::var("TEST").is_ok();

    let total_p1 = part1(&map, &pos);
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
