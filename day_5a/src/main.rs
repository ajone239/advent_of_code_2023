use std::{io, ops::Range};

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lines();

    let seeds = lines.next().unwrap().unwrap();

    let seeds: Vec<i64> = seeds.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut range = vec![];
    let mut ranges = vec![];

    lines.next();

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            ranges.push(range.clone());
            range.clear();
            continue;
        }

        if line.contains("map") {
            continue;
        }

        range.push(line);
    }
    ranges.push(range);

    let entries: Vec<Entry> = ranges
        .into_iter()
        .map(|rs| Entry::new_from_str_vec(rs))
        .collect();

    let min_location = seeds
        .into_iter()
        .map(|s| entries.iter().fold(s, |seed, entry| entry.map(seed)))
        .min()
        .unwrap();

    println!("location: {:?}", min_location);
}

#[derive(Debug)]
struct RangeMap {
    source: Range<i64>,
    destination: Range<i64>,
}

impl RangeMap {
    fn new_from_str(s: &str) -> Self {
        let vals: Vec<i64> = s
            .split(' ')
            .into_iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let dst = vals[0];
        let src = vals[1];
        let range = vals[2];

        Self {
            source: (src..src + range),
            destination: (dst..dst + range),
        }
    }
    fn map(&self, src: i64) -> Option<i64> {
        if self.source.contains(&src) {
            let diff = self.destination.start - self.source.start;
            Some(src + diff)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Entry {
    range_maps: Vec<RangeMap>,
}

impl Entry {
    fn new_from_str_vec(data: Vec<String>) -> Self {
        Self {
            range_maps: data
                .into_iter()
                .map(|s| RangeMap::new_from_str(&s))
                .collect(),
        }
    }
    fn map(&self, src: i64) -> i64 {
        for rm in &self.range_maps {
            if let Some(dst) = rm.map(src) {
                return dst;
            }
        }
        src
    }
}
