use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<u64> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1() -> u64 {
    let mut stones = get_input();
    let mut res = Vec::new();
    for _ in 0..25 {
        res = Vec::new();
        for stone in stones {
            let s = stone.to_string();
            match stone {
                0 => res.push(1),
                _ if s.len() % 2 == 0 => {
                    res.push(s[..s.len() / 2].parse().unwrap());
                    res.push(s[s.len() / 2..].parse().unwrap());
                }
                _ => {
                    res.push(stone * 2024);
                }
            }
        }
        stones = res.clone();
    }

    res.len() as u64
}

pub fn part2() -> u64 {
    let stones = get_input();
    let mut map: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        *map.entry(stone).or_default() += 1;
    }
    for _ in 0..75 {
        let mut map_copy = HashMap::new();
        for (stone, count) in map {
            let digits = f64::floor(f64::log10(stone as f64)) as u64 + 1;
            match stone {
                0 => *map_copy.entry(1).or_default() += count,
                _ if digits % 2 == 0 => {
                    *map_copy
                        .entry(stone / (u64::pow(10, (digits / 2) as u32)))
                        .or_default() += count;
                    *map_copy
                        .entry(stone % (u64::pow(10, (digits / 2) as u32)))
                        .or_default() += count;
                }
                _ => {
                    *map_copy.entry(stone * 2024).or_default() += count;
                }
            }
        }
        map = map_copy
    }

    map.values().sum()
}
