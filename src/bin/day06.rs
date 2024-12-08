use std::{collections::HashMap, collections::HashSet, fs};

const TEST_SOLUTION_PART1: i32 = 41;
const TEST_SOLUTION_PART2: i32 = 6;

const KNOWN_SOLUTION_PART1: i32 = 5312;
const KNOWN_SOLUTION_PART2: i32 = 0;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(map: &Map, starting_pos: &Pos) -> (HashMap<(i32, i32), i32>, i32) {
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
    let ntot = visited.len() as i32;
    (visited, ntot)
}

enum Trip {
    Exits,
    Loops,
}

const DELTA_XY: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // must start with UP

fn find_path(map: Map, starting_pos: Pos) -> Trip {
    let mut visited: HashSet<(Direction, i32, i32)> = HashSet::new();
    let mut direction = Direction::Up;
    let xdim = map[0].len() as i32;
    let ydim = map.len() as i32;

    let mut pos = starting_pos;

    loop {
        visited.insert((direction, pos.0, pos.1));

        let mut next = match direction {
            Direction::Up => (pos.0, pos.1 - 1) as Pos,
            Direction::Down => (pos.0, pos.1 + 1) as Pos,
            Direction::Left => (pos.0 - 1, pos.1) as Pos,
            Direction::Right => (pos.0 + 1, pos.1) as Pos,
        };

        if next.0 < 0 || next.0 >= xdim || next.1 < 0 || next.1 >= ydim {
            break Trip::Exits;
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

        if visited.contains(&(direction, next.0, next.1)) {
            return Trip::Loops;
        }

        pos = next;
    }
}

fn sim_guard(map: Map, starting_pos: Pos) -> Trip {
    let max_y = map.len() as i32;
    let max_x = map[0].len() as i32;
    let mut x = starting_pos.0;
    let mut y = starting_pos.1;
    let mut dir = 0; // UP
    let mut visited_pos = HashSet::new();
    let mut visited_pos_dir = HashSet::new();

    'outer: loop {
        visited_pos.insert((x, y));
        visited_pos_dir.insert((x, y, dir));
        let (dx, dy) = DELTA_XY[dir];
        let x1 = x + dx;
        let y1 = y + dy;
        if y1 < 0 || y1 >= max_y || x1 < 0 || x1 >= max_x {
            break 'outer;
        }
        if map[y1 as usize][x1 as usize] {
            dir = (dir + 1) % 4;
            continue;
        }
        x = x1;
        y = y1;
        if visited_pos_dir.contains(&(x, y, dir)) {
            return Trip::Loops;
        }
    }
    return Trip::Exits;
}

fn print_map(map: &Map, starting_pos: &Pos, visited: &HashMap<(i32, i32), i32>) {
    for (y, line) in map.iter().enumerate() {
        for (x, element) in line.iter().enumerate() {
            let symbol = if *starting_pos == (x as i32, y as i32) {
                '^'
            } else if *element {
                '#'
            } else if visited.get(&(x as i32, y as i32)).is_some() {
                'X'
            } else {
                '.'
            };
            print!("{symbol}");
        }
        println!("");
    }
}

fn part2(map: &Map, starting_pos: &Pos, visited: &HashMap<(i32, i32), i32>) -> i32 {
    let ntot = visited.len();
    let mut nit = 0;

    let mut loops = 0;
    // brute forcing it the stupid way (and apparently wrong)
    for (k, _) in visited {
        let mut blocked = map.clone();
        blocked[k.1 as usize][k.0 as usize] = true;
        // match sim_guard(blocked, *starting_pos) {
        match find_path(blocked, *starting_pos) {
            Trip::Loops => loops += 1,
            Trip::Exits => (),
        }
        nit += 1;
        println!("{} {} || {}/{}", k.0, k.1, nit, ntot);
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
