use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<Vec<i32>> {
    let file = File::open("input.txt").expect("Couldn't open file");
    let reader = BufReader::new(file);
    let mut input = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Couldn't read line");
        let mut vec = Vec::new();
        for part in line.split_whitespace() {
            vec.push(part.parse::<i32>().expect("Couldn't parse string"))
        }
        input.push(vec);
    }

    input
}

pub fn part1() -> i32 {
    let input = get_input();

    let mut ans = 0;
    for outer_report in input {
        let mut good = false;
        for i in 0..outer_report.len() {
            let mut report = outer_report.clone();
            report.remove(i);

            if report.len() == 1 {
                ans += 1;
                continue;
            }

            let increasing = report[1] > report[0];
            let mut prev = report[0];
            let mut iter = report.into_iter().skip(1).peekable();
            while let Some(level) = iter.next() {
                if (level > prev) != increasing {
                    break;
                }
                if level.abs_diff(prev) < 1 || level.abs_diff(prev) > 3 {
                    break;
                }
                prev = level;

                if iter.peek().is_none() {
                    good = true;
                }
            }
            if good {
                break;
            }
        }

        if good {
            ans += 1;
        }
    }

    ans
}

pub fn part2() -> i32 {
    todo!()
}
