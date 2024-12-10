use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<Vec<u32>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect()
        })
        .collect()
}

pub fn part1() -> u32 {
    let map = get_input();
    let (n, m) = (map.len(), map[0].len());

    let mut ans = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != 0 {
                continue;
            }

            let mut stack = vec![(i, j)];
            let mut visited = vec![vec![false; m]; n];
            while let Some((a, b)) = stack.pop() {
                if visited[a][b] {
                    continue;
                }
                visited[a][b] = true;
                if map[a][b] == 9 {
                    ans += 1;
                }

                if a > 0 && map[a - 1][b] == map[a][b] + 1 {
                    stack.push((a - 1, b));
                }
                if a < n - 1 && map[a + 1][b] == map[a][b] + 1 {
                    stack.push((a + 1, b));
                }
                if b > 0 && map[a][b - 1] == map[a][b] + 1 {
                    stack.push((a, b - 1));
                }
                if b < m - 1 && map[a][b + 1] == map[a][b] + 1 {
                    stack.push((a, b + 1));
                }
            }
        }
    }

    ans
}

pub fn part2() -> u32 {
    let map = get_input();
    let (n, m) = (map.len(), map[0].len());

    let mut ans = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != 0 {
                continue;
            }

            let mut stack = vec![(i, j)];
            while let Some((a, b)) = stack.pop() {
                if map[a][b] == 9 {
                    ans += 1;
                    continue;
                }

                if a > 0 && map[a - 1][b] == map[a][b] + 1 {
                    stack.push((a - 1, b));
                }
                if a < n - 1 && map[a + 1][b] == map[a][b] + 1 {
                    stack.push((a + 1, b));
                }
                if b > 0 && map[a][b - 1] == map[a][b] + 1 {
                    stack.push((a, b - 1));
                }
                if b < m - 1 && map[a][b + 1] == map[a][b] + 1 {
                    stack.push((a, b + 1));
                }
            }
        }
    }

    ans
}
