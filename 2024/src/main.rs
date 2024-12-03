use solutions::{day_1, day_2};

/// Controls which day's solution is executed.
const DAY: u8 = 2;
const PART: u8 = 1;

fn main() {
    let ans = solve(DAY, PART);
    println!("{ans}");
}

// TODO: make this generic.
fn solve(day: u8, part: u8) -> i32 {
    match (day, part) {
        (1, 1) => day_1::part1(),
        (1, 2) => day_1::part2(),
        (2, 1) => day_2::part1(),
        (2, 2) => day_2::part2(),
        _ => 0,
    }
}

const INPUT_PATH: &str = "input.txt";

mod solutions;
