use solutions::{day_1, day_2, day_3, day_4};

/// Controls which day's solution is executed.
const DAY: u8 = 4;
const PART: u8 = 2;

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
        (3, 1) => day_3::part1(),
        (3, 2) => day_3::part2(),
        (4, 1) => day_4::part1(),
        (4, 2) => day_4::part2(),
        _ => 0,
    }
}

mod solutions;
