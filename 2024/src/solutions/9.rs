use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<i128> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i128)
        .collect()
}

pub fn part1() -> i128 {
    let input = get_input();

    let mut res: Vec<i128> = Vec::new();
    for (i, n) in input.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*n {
                res.push(i as i128 / 2);
            }
        } else {
            for _ in 0..*n {
                res.push(-1);
            }
        }
    }

    let mut left: usize = 0;
    let mut right: usize = res.len() - 1;
    while left < right {
        while res[left] != -1 {
            left += 1;
        }
        while res[right] == -1 {
            right -= 1;
        }
        res[left] = res[right];
        res[right] = -1;
        left += 1;
        right -= 1;
    }

    let mut ans = 0;
    let mut curr = 0;
    for n in res.iter() {
        if *n == -1 {
            continue;
        }
        ans += n * curr;
        curr += 1;
    }

    ans
}

pub fn part2() -> i128 {
    let input = get_input();

    let mut res: Vec<i128> = Vec::new();
    for (i, n) in input.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*n {
                res.push(i as i128 / 2);
            }
        } else {
            for _ in 0..*n {
                res.push(-1);
            }
        }
    }

    let mut free_spaces: VecDeque<(i128, i128)> = VecDeque::new();
    let mut files: VecDeque<(i128, i128, i128)> = VecDeque::new();
    let mut i = 0;
    while i < res.len() {
        let mut length = 0;
        let n = res[i];
        while i < res.len() && res[i] == n {
            length += 1;
            i += 1;
        }
        match n {
            -1 => {
                free_spaces.push_back(((i - length) as i128, length as i128));
            }
            _ => {
                files.push_back(((i - length) as i128, length as i128, res[i - 1]));
            }
        }
    }

    while let Some(file) = files.pop_back() {
        let mut i = 0;
        while i < free_spaces.len() && free_spaces[i].1 < file.1 {
            i += 1;
        }
        if i == free_spaces.len() || free_spaces[i].0 > file.0 {
            continue;
        }

        for j in 0..file.1 {
            res[(free_spaces[i].0 + j) as usize] = file.2;
            res[(file.0 + j) as usize] = -1;
        }
        free_spaces[i].1 -= file.1;
        free_spaces[i].0 += file.1;
    }

    for n in res.iter_mut().rev() {
        if *n == -1 {
            *n = -2;
        } else {
            break;
        }
    }

    let mut ans = 0;
    let mut curr = 0;
    for n in res.iter() {
        match *n {
            -2 => {
                break;
            }
            -1 => {
                curr += 1;
            }
            _ => {
                ans += n * curr;
                curr += 1;
            }
        }
    }

    ans
}
