use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut output = Vec::new();
    for line in reader.lines() {
        output.push(line.unwrap().chars().collect::<Vec<_>>());
    }

    output
}

pub fn part1() -> i32 {
    let input = get_input();
    let n = input.len();
    let m = input.first().unwrap().len();

    let mut ans = 0;

    // Horizontal
    for i in 0..n {
        for j in 0..=m - 4 {
            let s1 = format!(
                "{}{}{}{}",
                input[i][j],
                input[i][j + 1],
                input[i][j + 2],
                input[i][j + 3]
            );
            let s2 = format!(
                "{}{}{}{}",
                input[i][j + 3],
                input[i][j + 2],
                input[i][j + 1],
                input[i][j]
            );
            if s1 == "XMAS" || s2 == "XMAS" {
                ans += 1;
            }
        }
    }
    // Vertical
    for i in 0..=n - 4 {
        for j in 0..m {
            let s1 = format!(
                "{}{}{}{}",
                input[i][j],
                input[i + 1][j],
                input[i + 2][j],
                input[i + 3][j]
            );
            let s2 = format!(
                "{}{}{}{}",
                input[i + 3][j],
                input[i + 2][j],
                input[i + 1][j],
                input[i][j]
            );
            if s1 == "XMAS" || s2 == "XMAS" {
                ans += 1;
            }
        }
    }
    // Diagonal, top left to bottom right
    for i in 0..=n - 4 {
        for j in 0..=m - 4 {
            let s1 = format!(
                "{}{}{}{}",
                input[i][j],
                input[i + 1][j + 1],
                input[i + 2][j + 2],
                input[i + 3][j + 3]
            );
            let s2 = format!(
                "{}{}{}{}",
                input[i + 3][j + 3],
                input[i + 2][j + 2],
                input[i + 1][j + 1],
                input[i][j]
            );
            if s1 == "XMAS" || s2 == "XMAS" {
                ans += 1;
            }
        }
    }
    // Diagonal, bottom left to top right
    for i in 3..n {
        for j in 0..=m - 4 {
            let s1 = format!(
                "{}{}{}{}",
                input[i][j],
                input[i - 1][j + 1],
                input[i - 2][j + 2],
                input[i - 3][j + 3]
            );
            let s2 = format!(
                "{}{}{}{}",
                input[i - 3][j + 3],
                input[i - 2][j + 2],
                input[i - 1][j + 1],
                input[i][j]
            );
            if s1 == "XMAS" || s2 == "XMAS" {
                ans += 1;
            }
        }
    }

    ans
}

pub fn part2() -> i32 {
    let input = get_input();
    let n = input.len();
    let m = input.first().unwrap().len();

    let mut ans = 0;

    for i in 0..n - 2 {
        for j in 0..m - 2 {
            // top left to bottom right
            let s1 = format!(
                "{}{}{}",
                input[i][j],
                input[i + 1][j + 1],
                input[i + 2][j + 2]
            );
            // bottom left to top right
            let s2 = format!(
                "{}{}{}",
                input[i + 2][j],
                input[i + 1][j + 1],
                input[i][j + 2]
            );

            if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
                ans += 1
            }
        }
    }

    ans
}
