use std::{collections::HashMap, io};

use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lines();

    // Directions line
    let directions = lines.next().unwrap().unwrap();

    let directions: Vec<Direction> = directions.chars().map(|c| Direction::new(c)).collect();

    // nil line
    let _ = lines.next().unwrap();

    let mut map = HashMap::new();

    for line in lines {
        let line = line.unwrap();

        // Ex. AAA = (BBB, CCC)
        let re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();

        let caps = re.captures(&line).unwrap();

        let node = (&caps[1]).to_string();
        let left = (&caps[2]).to_string();
        let right = (&caps[3]).to_string();

        map.insert(node, Node { left, right });
    }

    // get the length of all the cycles in the map for zhjj
    let steps: Vec<u64> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .into_iter()
        .map(|c| find_z(c, &map, &directions))
        .collect();

    println!("Steps: {}", lcm(steps));
}

fn find_z(start: &String, map: &HashMap<String, Node>, directions: &Vec<Direction>) -> u64 {
    let mut count = 0;

    let mut dir_iter = directions.iter();

    let mut curr = start;

    while !curr.ends_with('Z') {
        match dir_iter.next() {
            Some(dir) => {
                count += 1;

                let node = map.get(curr).unwrap();

                curr = match dir {
                    Direction::Left => &node.left,
                    Direction::Right => &node.right,
                }
            }
            None => dir_iter = directions.iter(),
        }
    }
    count
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

struct Node {
    left: String,
    right: String,
}

fn lcm(nums: Vec<u64>) -> u64 {
    let factors = nums.into_iter().map(factorize);

    let lcm_factors = factors.fold(HashMap::new(), |a, b| merge(a, b));

    lcm_factors.into_iter().fold(1, |acc, (val, count)| {
        let mut acc = acc;
        for _ in 0..count {
            acc *= val
        }
        acc
    })
}

fn merge(a: HashMap<u64, u64>, b: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut rv = HashMap::new();

    for (k, av) in a.iter() {
        if let Some(bv) = b.get(k) {
            rv.insert(*k, u64::max(*av, *bv));
        } else {
            rv.insert(*k, *av);
        }
    }

    for (k, bv) in b.iter() {
        if let Some(_) = a.get(k) {
            continue;
        } else {
            rv.insert(*k, *bv);
        }
    }

    rv
}

fn factorize(n: u64) -> HashMap<u64, u64> {
    let mut n = n;
    let root_n = (n as f64).sqrt().ceil() as u64 + 1;
    let mut rv = HashMap::new();
    for factor in 2..root_n {
        if n % factor != 0 {
            continue;
        }
        let mut count = 0;
        while n % factor == 0 {
            n /= factor;
            count += 1;
        }
        rv.insert(factor, count);
    }
    if n > 1 {
        rv.insert(n, 1);
    }

    rv
}
