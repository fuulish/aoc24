use std::fs;

fn main() {
    println!("Day 2");
    let mut args = std::env::args();
    args.next().expect("no program name?");
    let fname = args.next().expect("provide input file name");
    let data = fs::read_to_string(fname).unwrap();

    let safe = data
        .lines()
        .map(|s| {
            let itr = s.split_whitespace();

            let desc: Vec<_> = itr
                .clone()
                .zip(itr.skip(1))
                .map(|(a, b)| a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap())
                .collect();

            let nsign: i32 = desc.iter().map(|v| v.signum()).sum();
            println!("{nsign}");
            let monotonuous = nsign.abs() as usize == desc.len();

            println!("{monotonuous}");
            println!("{:?}", desc);
            monotonuous
                && (desc
                    .iter()
                    .map(|&v| v.abs())
                    .filter(|&v| v >= 1 && v <= 3)
                    .count()
                    == desc.len())
        })
        .count();

    println!("{safe} safe");
    assert!(safe == 2);
}
