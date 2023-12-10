use std::{fmt::Debug, io};

fn main() {
    let stdin = io::stdin();

    let lines: Vec<Vec<char>> = stdin
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut part_number: Option<String> = None;
    let mut part_start: Option<Point> = None;
    let mut parts: Vec<Part> = vec![];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                match part_number {
                    Some(ref mut s) => s.push(*c),
                    None => {
                        part_number = Some(c.to_string());
                        part_start = Some(Point { x: j, y: i });
                    }
                }
            }

            if !c.is_ascii_digit() || j == line.len() - 1 {
                if let (Some(number), Some(start)) = (part_number.take(), part_start.take()) {
                    // TODO: fix for edge corner case
                    let j = if j == 0 { line.len() } else { j };

                    let end = Point { x: j - 1, y: i };
                    let number: u64 = number.parse().unwrap();

                    let part = Part {
                        left: start,
                        right: end,
                        part_number: number,
                    };
                    parts.push(part);
                }
            }
        }
    }

    let total: u64 = parts
        .iter()
        .filter(|p| p.is_part_good(&lines))
        .map(|p| p.part_number)
        .sum();

    println!("Total: {}", total);
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Part {
    left: Point,
    right: Point,
    part_number: u64,
}

impl Part {
    fn is_part_good(&self, scheme: &Vec<Vec<char>>) -> bool {
        //     North
        //     .....
        // West.###. East
        //     .....
        //     South
        //
        let lx = if self.left.x == 0 { 0 } else { self.left.x - 1 };

        let is_c_good = |c: char| -> bool { c != '.' && !c.is_ascii_digit() };

        for j in (lx)..=(self.right.x + 1) {
            // North
            let i = self.left.y.overflowing_sub(1).0;
            if let Some(c) = read_checked(&scheme, i, j) {
                if is_c_good(c) {
                    return true;
                }
            }

            // South
            let i = self.left.y + 1;
            if let Some(c) = read_checked(&scheme, i, j) {
                if is_c_good(c) {
                    return true;
                }
            }
        }

        let ly = if self.left.y == 0 { 0 } else { self.left.y - 1 };

        for i in (ly)..=(self.right.y + 1) {
            // West
            let j = self.left.x.overflowing_sub(1).0;
            if let Some(c) = read_checked(&scheme, i, j) {
                if is_c_good(c) {
                    return true;
                }
            }

            // East
            let j = self.right.x + 1;
            if let Some(c) = read_checked(&scheme, i, j) {
                if is_c_good(c) {
                    return true;
                }
            }
        }

        false
    }
}

fn read_checked<T: Copy + Debug>(matrix: &Vec<Vec<T>>, i: usize, j: usize) -> Option<T> {
    if i >= matrix.len() || j >= matrix[0].len() {
        return None;
    }
    let rv = matrix[i][j];
    Some(rv)
}
