use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<Vec<u128>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let nums = line.split(" ");
        let mut v = Vec::new();
        for n in nums {
            v.push(
                n.chars()
                    .filter(|c| *c != ':')
                    .collect::<String>()
                    .parse::<u128>()
                    .unwrap(),
            );
        }
        vec.push(v);
    }
    vec
}

pub fn part1() -> u128 {
    let input = get_input();
    let mut ans = 0;
    for v in input {
        let target = v[0];
        let nums = v[1..].to_vec();
        if helper1(&nums, target, nums[0], 1) {
            ans += target;
        }
    }
    ans
}

fn helper1(nums: &Vec<u128>, target: u128, total: u128, i: usize) -> bool {
    if i == nums.len() {
        return total == target;
    }

    helper1(nums, target, total + nums[i], i + 1) || helper1(nums, target, total * nums[i], i + 1)
}

pub fn part2() -> u128 {
    let input = get_input();
    let mut ans = 0;

    for v in input {
        let target = v[0];
        let nums = v[1..].to_vec();
        if helper2(&nums, target, nums[0], 1) {
            ans += target;
        }
    }

    ans
}

fn helper2(nums: &Vec<u128>, target: u128, total: u128, i: usize) -> bool {
    if i == nums.len() {
        return total == target;
    }

    let mut res = false;
    if let Some(n) = total.checked_add(nums[i]) {
        res = res || helper2(nums, target, n, i + 1);
    }
    if let Some(n) = total.checked_mul(nums[i]) {
        res = res || helper2(nums, target, n, i + 1);
    }
    if let Ok(n) = format!("{}{}", total, nums[i]).parse::<u128>() {
        res = res || helper2(nums, target, n, i + 1);
    }

    res
}
