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
        let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

        let caps = re.captures(&line).unwrap();

        let node = (&caps[1]).to_string();
        let left = (&caps[2]).to_string();
        let right = (&caps[3]).to_string();

        map.insert(node, Node { left, right });
    }

    let mut curr = "AAA";
    let mut count = 0;

    let mut dir_iter = directions.iter();

    while curr != "ZZZ" {
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

    println!("Steps: {}", count);
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
