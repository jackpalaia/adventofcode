use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        vec.push(line.chars().collect::<Vec<char>>());
    }
    vec
}

pub fn part1() -> i32 {
    let mut grid = get_input();
    let (n, m) = (grid.len(), grid[0].len());

    let (mut x, mut y, mut dir) = (0, 0, 0);
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '#' && grid[i][j] != '.' {
                x = i;
                y = j;
                for (i, c) in ['^', '>', 'v', '<'].iter().enumerate() {
                    if grid[i][j] == *c {
                        dir = i;
                        break;
                    }
                }
            }
        }
    }

    let mut visited = 0;
    loop {
        if grid[x][y] != 'X' {
            grid[x][y] = 'X';
            visited += 1;
        }
        let mut moved = false;
        while !moved {
            match dir {
                0 => {
                    if x > 0 && grid[x - 1][y] == '#' {
                        dir = 1;
                    } else {
                        if x == 0 {
                            return visited;
                        }
                        x -= 1;
                        moved = true;
                    }
                }
                1 => {
                    if y + 1 < m && grid[x][y + 1] == '#' {
                        dir = 2;
                    } else {
                        if y == m - 1 {
                            return visited;
                        }
                        y += 1;
                        moved = true;
                    }
                }
                2 => {
                    if x + 1 < n && grid[x + 1][y] == '#' {
                        dir = 3;
                    } else {
                        if x == n - 1 {
                            return visited;
                        }
                        x += 1;
                        moved = true;
                    }
                }
                3 => {
                    if y > 0 && grid[x][y - 1] == '#' {
                        dir = 0;
                    } else {
                        if y == 0 {
                            return visited;
                        }
                        y -= 1;
                        moved = true;
                    }
                }
                _ => break,
            }
        }
    }
}

pub fn part2() -> i32 {
    let mut grid = get_input();
    let (n, m) = (grid.len(), grid[0].len());
    let mut count = 0;

    let (mut x, mut y, mut dir) = (0, 0, 0);
    for i in 0..n {
        for j in 0..m {
            if ['^', '>', 'v', '<'].contains(&grid[i][j]) {
                x = i;
                y = j;
                dir = ['^', '>', 'v', '<']
                    .iter()
                    .position(|c| c == &grid[i][j])
                    .unwrap()
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if i == x && j == y {
                continue;
            }
            if grid[i][j] == '.' {
                grid[i][j] = '#';
                count += search(&grid, x, y, dir) as i32;
                grid[i][j] = '.';
            }
        }
    }

    count
}

fn search(grid: &[Vec<char>], mut x: usize, mut y: usize, mut dir: usize) -> bool {
    let (n, m) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![0; m]; n];

    loop {
        if (visited[x][y] & (1 << dir)) > 0 {
            return true;
        }
        visited[x][y] |= 1 << dir;

        let mut moved = false;
        while !moved {
            match dir {
                0 => {
                    if x == 0 {
                        return false;
                    }
                    if grid[x - 1][y] == '#' {
                        dir = 1;
                    } else {
                        x -= 1;
                        moved = true;
                    }
                }
                1 => {
                    if y == m - 1 {
                        return false;
                    }
                    if grid[x][y + 1] == '#' {
                        dir = 2;
                    } else {
                        y += 1;
                        moved = true;
                    }
                }
                2 => {
                    if x == n - 1 {
                        return false;
                    }
                    if grid[x + 1][y] == '#' {
                        dir = 3;
                    } else {
                        x += 1;
                        moved = true;
                    }
                }
                3 => {
                    if y == 0 {
                        return false;
                    }
                    if grid[x][y - 1] == '#' {
                        dir = 0;
                    } else {
                        y -= 1;
                        moved = true;
                    }
                }
                _ => {}
            }
        }
    }
}
