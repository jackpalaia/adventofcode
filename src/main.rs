/// Controls which day's solution is executed.
const DAY: u8 = 1;
const PART: u8 = 2;

fn main() {
    let ans = solve(DAY, PART);
    println!("{ans}");
}

// TODO: make this generic.
fn solve(day: u8, part: u8) -> i32 {
    match (day, part) {
        (1, 1) => mod_1::part1(),
        (1, 2) => mod_1::part2(),
        _ => 0,
    }
}

const INPUT_PATH: &str = "input.txt";

#[path = "1.rs"]
mod mod_1;
