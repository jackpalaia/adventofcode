use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<(i32, i32)> {
    let f = File::open("input.txt").expect("Input file could not be opened");
    let reader = BufReader::new(f);

    let mut vec = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Couldn't read line");

        let mut parts = line.split_whitespace();
        let num1 = parts
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Couldn't parse num1");
        let num2 = parts
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Couldn't parse num2");
        vec.push((num1, num2));
    }

    vec
}

pub fn part1() -> i32 {
    let input = get_input();

    let mut nums1 = input.iter().map(|t| t.0).collect::<Vec<_>>();
    let mut nums2 = input.iter().map(|t| t.1).collect::<Vec<_>>();
    nums1.sort();
    nums2.sort();

    let mut ans = 0;
    for (num1, num2) in nums1.iter().zip(nums2.iter()) {
        let diff = num1 - num2;
        ans += diff.abs();
    }

    ans
}

pub fn part2() -> i32 {
    let input = get_input();

    let nums1 = input.iter().map(|t| t.0);
    let nums2 = input.iter().map(|t| t.1);

    let mut freqs: HashMap<i32, i32> = HashMap::new();
    for n in nums2 {
        *freqs.entry(n).or_default() += 1;
    }

    let mut ans = 0;
    for n in nums1 {
        ans += n * *freqs.entry(n).or_default();
    }

    ans
}
