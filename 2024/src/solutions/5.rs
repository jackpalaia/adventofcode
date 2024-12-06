use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, File},
    io::{BufRead, BufReader, Read},
};

fn get_input(part2: bool) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let s = fs::read_to_string("input.txt").unwrap();
    let mut parts = s.split("\n\n");

    let orderings = parts.next().unwrap();
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in orderings.lines() {
        let mut pages = line.split('|');
        let from = pages.next().unwrap().parse::<i32>().unwrap();
        let to = pages.next().unwrap().parse::<i32>().unwrap();

        if part2 {
            map.entry(from).or_default().insert(to);
        } else {
            map.entry(to).or_default().insert(from);
        }
    }

    let updates = parts.next().unwrap();
    let vec = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (map, vec)
}

pub fn part1() -> i32 {
    let (mut orderings, updates) = get_input(false);

    let mut ans: i32 = 0;
    for update in updates {
        let mut cant_see: HashSet<i32> = HashSet::new();
        let mut bad = false;
        for n in update.clone() {
            if cant_see.contains(&n) {
                bad = true;
                break;
            }
            cant_see.extend(orderings.entry(n).or_default().clone());
        }
        if bad {
            continue;
        }

        ans += update[update.len() / 2]
    }

    ans
}

pub fn part2() -> i32 {
    let (mut orderings, updates) = get_input(true);
    dbg!(orderings.clone());

    let mut ans = 0;
    for update in updates {
        let mut cant_see: HashSet<i32> = HashSet::new();
        let mut bad = false;
        for n in update.clone() {
            if cant_see.contains(&n) {
                bad = true;
                break;
            }
            cant_see.extend(orderings.entry(n).or_default().clone());
        }
        if !bad {
            continue;
        }
        let updates_set: HashSet<i32> = HashSet::from_iter(update.clone());

        let mut indegrees: HashMap<i32, i32> = HashMap::new();
        for ordering in orderings.clone() {
            indegrees.entry(ordering.0).or_default();
            for n in ordering.1 {
                if updates_set.contains(&ordering.0) && updates_set.contains(&n) {
                    *indegrees.entry(n).or_default() += 1;
                }
            }
        }
        dbg!(indegrees.clone());

        let mut queue: VecDeque<i32> = VecDeque::new();
        for i in indegrees.clone() {
            if i.1 == 0 {
                queue.push_back(i.0);
            }
        }

        let mut v = Vec::new();
        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            v.push(curr);

            for n in orderings.get(&curr).unwrap() {
                *indegrees.entry(*n).or_default() -= 1;
                if indegrees[n] == 0 {
                    queue.push_back(*n);
                }
            }
        }

        let mut output = Vec::new();
        let mut nums = HashSet::new();
        for n in update {
            nums.insert(n);
        }

        for n in v.clone() {
            if nums.contains(&n) {
                output.push(n);
            }
        }

        ans += output[output.len() / 2];
    }

    ans
}
