use std::fs;

fn main() {
    println!("Day 2");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let safe = data
        .lines()
        .filter(|s| {
            // XXX: could collect here, but that means having to create a new iterator later on
            let itr = s.split_whitespace().map(|v| v.parse::<i32>().unwrap());

            let desc: Vec<_> = itr.clone().zip(itr.skip(1)).map(|(a, b)| b - a).collect();

            let nsign: i32 = desc.iter().map(|v| v.signum()).sum();
            let monotonuous = nsign.abs() as usize == desc.len();

            let num_steps = desc.len();
            let desc: Vec<i32> = desc
                .iter()
                .map(|&v| v.abs())
                .filter(|&v| v >= 1 && v <= 3)
                .collect();

            monotonuous && num_steps == desc.len()
        })
        .count();

    println!("{safe} safe");
    assert!(safe == 631);
}
