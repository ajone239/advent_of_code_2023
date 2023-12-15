use std::io;

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lines();

    let times = lines.next().unwrap().unwrap();
    let distances = lines.next().unwrap().unwrap();

    let times = colon_then_fields(&times);
    let distances = colon_then_fields(&distances);

    let time = times.join("").parse().unwrap();
    let record_distance = distances.join("").parse().unwrap();

    let race = Race {
        time,
        record_distance,
    };

    let total = race.count_wins();

    println!("Total: {}", total);
}

fn colon_then_fields(s: &str) -> Vec<&str> {
    let fields: Vec<&str> = s.split(':').collect();
    let fields = fields[1];
    fields
        .split_ascii_whitespace()
        // .map(|f| f.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Race {
    time: i64,
    record_distance: i64,
}

impl Race {
    fn new(race: (i64, i64)) -> Self {
        Self {
            time: race.0,
            record_distance: race.1,
        }
    }

    fn count_wins(&self) -> i64 {
        let mut wins = 0;
        for hold_time in 1..self.time {
            let distance = hold_time * (self.time - hold_time);

            if distance > self.record_distance {
                wins += 1;
            }
        }
        wins
    }
}
