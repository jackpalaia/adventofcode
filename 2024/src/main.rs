use std::env;

use solutions::{day_1, day_2, day_3, day_4, day_5, day_6};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().unwrap();
    let part = args[2].parse::<u8>().unwrap();

    let ans = solve(day, part);
    println!("{ans}");
}

// TODO: make this generic.
fn solve(day: u8, part: u8) -> i32 {
    match (day, part) {
        (1, 1) => day_1::part1(),
        (1, 2) => day_1::part2(),
        (2, 1) => day_2::part1(),
        (2, 2) => day_2::part2(),
        (3, 1) => day_3::part1(),
        (3, 2) => day_3::part2(),
        (4, 1) => day_4::part1(),
        (4, 2) => day_4::part2(),
        (5, 1) => day_5::part1(),
        (5, 2) => day_5::part2(),
        (6, 1) => day_6::part1(),
        (6, 2) => day_6::part2(),
        _ => 0,
    }
}

mod solutions;
