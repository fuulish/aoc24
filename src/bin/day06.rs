use std::{collections::HashSet, fs};

const TEST_SOLUTION_PART1: i32 = 41;
const TEST_SOLUTION_PART2: i32 = 6;

const KNOWN_SOLUTION_PART1: i32 = 5312;
const KNOWN_SOLUTION_PART2: i32 = 1748;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(map: &Map, starting_pos: &Pos) -> (HashSet<(Direction, i32, i32)>, i32) {
    let (_, visited) = find_path(map, starting_pos);

    let mut wo_dir = HashSet::new();
    for v in visited.iter() {
        wo_dir.insert((v.1, v.2));
    }
    let ntot = wo_dir.len() as i32;

    (visited, ntot)
}

enum Trip {
    Exits,
    Loops,
}

fn find_path(map: &Map, starting_pos: &Pos) -> (Trip, HashSet<(Direction, i32, i32)>) {
    let mut visited: HashSet<(Direction, i32, i32)> = HashSet::new();
    let mut direction = Direction::Up;
    let xdim = map[0].len() as i32;
    let ydim = map.len() as i32;

    let mut pos = *starting_pos;

    let trip = loop {
        visited.insert((direction, pos.0, pos.1));

        let next = match direction {
            Direction::Up => (pos.0, pos.1 - 1) as Pos,
            Direction::Down => (pos.0, pos.1 + 1) as Pos,
            Direction::Left => (pos.0 - 1, pos.1) as Pos,
            Direction::Right => (pos.0 + 1, pos.1) as Pos,
        };

        if next.0 < 0 || next.0 >= xdim || next.1 < 0 || next.1 >= ydim {
            break Trip::Exits;
        }

        if map[next.1 as usize][next.0 as usize] {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            continue;
            // need to continue here, because we need to check it for collisions
        }

        if visited.contains(&(direction, next.0, next.1)) {
            return (Trip::Loops, visited);
        }

        pos = next;
    };

    (trip, visited)
}

fn print_map(map: &Map, starting_pos: &Pos, visited: &HashSet<(Direction, i32, i32)>) {
    let mut wo_dir = HashSet::new();
    for &v in visited {
        wo_dir.insert((v.1, v.2));
    }

    for (y, line) in map.iter().enumerate() {
        for (x, element) in line.iter().enumerate() {
            let symbol = if *starting_pos == (x as i32, y as i32) {
                '^'
            } else if *element {
                '#'
            } else if wo_dir.get(&(x as i32, y as i32)).is_some() {
                'X'
            } else {
                '.'
            };
            print!("{symbol}");
        }
        println!("");
    }
}

fn part2(map: &Map, starting_pos: &Pos, visited: &HashSet<(Direction, i32, i32)>) -> i32 {
    let mut loops = 0;
    let mut wo_dir = HashSet::new();
    for v in visited.iter() {
        wo_dir.insert((v.1, v.2));
    }
    wo_dir.remove(starting_pos);
    for (x, y) in &wo_dir {
        let mut blocked = map.clone();
        blocked[*y as usize][*x as usize] = true;
        match find_path(&blocked, starting_pos).0 {
            Trip::Loops => loops += 1,
            Trip::Exits => (),
        }
    }

    loops
}

type Map = Vec<Vec<bool>>;
type Pos = (i32, i32);

fn extract_input(data: &str) -> (Map, Pos) {
    let mut map = Map::new();
    let mut pos = (0, 0);
    for (i, line) in data.lines().enumerate() {
        let horizontal: Vec<bool> = line.chars().into_iter().map(|c| c == '#').collect();
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

    let (visited, total_p1) = part1(&map, &pos);
    print_map(&map, &pos, &visited);
    println!("{total_p1}");
    if use_test {
        assert!(total_p1 == TEST_SOLUTION_PART1);
    } else {
        assert!(total_p1 == KNOWN_SOLUTION_PART1);
    }

    let total_p2 = part2(&map, &pos, &visited);
    println!("{total_p2}");
    if use_test {
        assert!(total_p2 == TEST_SOLUTION_PART2);
    } else {
        assert!(total_p2 == KNOWN_SOLUTION_PART2);
    }
}
