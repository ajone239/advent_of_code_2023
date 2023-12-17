use std::io;

fn main() {
    let stdin = io::stdin();

    let sequences: Vec<Vec<i64>> = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<i64>>()
        })
        .collect();

    let sum: i64 = sequences.iter().map(|s| extend(s)).sum();

    println!("Total: {}", sum);
}

fn extend(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|v| *v == 0) {
        return 0;
    }

    let mut diff = vec![];

    for i in 0..seq.len() - 1 {
        diff.push(seq[i + 1] - seq[i]);
    }

    seq.iter().last().unwrap() + extend(&diff)
}
