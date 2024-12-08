use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn part1() -> u32 {
    let grid = get_input();
    let (n, m) = (grid.len(), grid[0].len());
    let mut antennas = HashSet::new();
    for row in &grid {
        for c in row {
            if *c != '.' && *c != '#' {
                antennas.insert(*c);
            }
        }
    }

    let mut antinode_locations = vec![vec![false; m]; n];
    let mut count = 0;
    for antenna in antennas {
        let mut positions = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == antenna {
                    positions.push((i as i32, j as i32));
                }
            }
        }

        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let x = positions[j].0 - positions[i].0;
                let y = positions[j].1 - positions[i].1;

                if check_bounds(positions[i].0 - x, positions[i].1 - y, n, m)
                    && !antinode_locations[(positions[i].0 - x) as usize]
                        [(positions[i].1 - y) as usize]
                {
                    antinode_locations[(positions[i].0 - x) as usize]
                        [(positions[i].1 - y) as usize] = true;
                    count += 1;
                }
                if check_bounds(positions[j].0 + x, positions[j].1 + y, n, m)
                    && !antinode_locations[(positions[j].0 + x) as usize]
                        [(positions[j].1 + y) as usize]
                {
                    antinode_locations[(positions[j].0 + x) as usize]
                        [(positions[j].1 + y) as usize] = true;
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_bounds(x: i32, y: i32, n: usize, m: usize) -> bool {
    x >= 0 && y >= 0 && x < n as i32 && y < m as i32
}

pub fn part2() -> u32 {
    let grid = get_input();
    let (n, m) = (grid.len(), grid[0].len());
    let mut antennas = HashSet::new();
    for row in &grid {
        for c in row {
            if *c != '.' && *c != '#' {
                antennas.insert(*c);
            }
        }
    }

    let mut antinode_locations = vec![vec![false; m]; n];
    let mut count = 0;
    for antenna in antennas {
        let mut positions = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == antenna {
                    positions.push((i as i32, j as i32));
                }
            }
        }

        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let orig_x = positions[j].0 - positions[i].0;
                let orig_y = positions[j].1 - positions[i].1;

                let mut x = 0;
                let mut y = 0;
                while check_bounds(positions[i].0 - x, positions[i].1 - y, n, m) {
                    if !antinode_locations[(positions[i].0 - x) as usize]
                        [(positions[i].1 - y) as usize]
                    {
                        antinode_locations[(positions[i].0 - x) as usize]
                            [(positions[i].1 - y) as usize] = true;
                        count += 1;
                    }

                    x += orig_x;
                    y += orig_y;
                }

                let mut x = 0;
                let mut y = 0;
                while check_bounds(positions[j].0 + x, positions[j].1 + y, n, m) {
                    if !antinode_locations[(positions[j].0 + x) as usize]
                        [(positions[j].1 + y) as usize]
                    {
                        antinode_locations[(positions[j].0 + x) as usize]
                            [(positions[j].1 + y) as usize] = true;
                        count += 1;
                    }

                    x += orig_x;
                    y += orig_y;
                }
            }
        }
    }

    for row in antinode_locations {
        for c in row {
            print!("{}", if c { '#' } else { '.' });
        }
        println!();
    }
    println!();

    count
}
