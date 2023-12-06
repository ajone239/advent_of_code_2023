use std::{cmp, io};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let true_red = 12;
    let true_green = 13;
    let true_blue = 14;

    let games = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Game::new_from_string(&l));

    let good_game_ids = games
        .filter(|g| g.red <= true_red && g.green <= true_green && g.blue <= true_blue)
        .map(|g| g.id);

    let total: u32 = good_game_ids.sum();

    println!("Total: {}", total);

    Ok(())
}

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new_from_string(s: &str) -> Self {
        // EX. Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let parts: Vec<&str> = s.split(':').collect();

        let game = parts[0];
        let game: Vec<&str> = game.split(' ').collect();
        let game_id = game[1];

        let id = game_id.parse::<u32>().unwrap();

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

        Self {
            id,
            red,
            green,
            blue,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_from_string() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game = Game::new_from_string(s);

        assert_eq!(game.id, 1);
        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 6);
    }
}
