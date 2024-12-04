use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn get_input() -> String {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut output = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        output += &line;
    }

    output
}

pub fn part1() -> i32 {
    let input = get_input();

    let mut ans = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();

        ans += a * b;
    }

    ans
}

// Should have probably just figured the regex out.
pub fn part2() -> i32 {
    let input = get_input();
    let s = input;

    let mut ans = 0;
    let mut parsed_input = String::new();
    let mut curr = 0;
    loop {
        if let Some(index) = s[curr..].find("don't()") {
            parsed_input += &s[curr..index + curr];
            curr += index;
        } else {
            parsed_input += &s[curr..];
            break;
        }
        if let Some(index) = s[curr..].find("do()") {
            curr += index;
        } else {
            break;
        }
    }

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [a, b]) in re.captures_iter(&parsed_input).map(|c| c.extract()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();

        ans += a * b;
    }

    ans
}
