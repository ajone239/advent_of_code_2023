use std::io;

fn main() {
    let stdin = io::stdin();

    let sequences: Vec<Vec<i64>> = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let sum: i64 = sequences.iter().map(|s| extend(s)).sum();

    println!("Total: {}", sum);
}

fn extend(seq: &Vec<i64>) -> i64 {
    if seq.windows(2).all(|w| w[0] == w[1]) {
        return seq[0];
    }

    let diff = seq.windows(2).map(|w| w[1] - w[0]).collect();

    seq.iter().last().unwrap() + extend(&diff)
}
