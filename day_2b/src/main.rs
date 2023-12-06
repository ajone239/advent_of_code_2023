use std::{cmp, io};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let games = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Game::new_from_string(&l));

    let total: u32 = games.map(|g| g.red * g.green * g.blue).sum();

    println!("Total: {}", total);

    Ok(())
}

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new_from_string(s: &str) -> Self {
        // EX. Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let parts: Vec<&str> = s.split(':').collect();

        let rounds = parts[1];

        let rounds = rounds.split(';');

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in rounds {
            let pulls = round.split(',');

            for pull in pulls {
                let draw: Vec<&str> = pull.split(' ').collect();

                let color = draw[2];
                let count = draw[1].parse::<u32>().unwrap();

                match color {
                    "red" => red = cmp::max(red, count),
                    "green" => green = cmp::max(green, count),
                    "blue" => blue = cmp::max(blue, count),
                    _ => continue,
                }
            }
        }

        Self { red, green, blue }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_from_string() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = Game::new_from_string(s);

        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 6);
    }
}
