use std::{collections::HashSet, io};

fn main() {
    let stdin = io::stdin();

    let total: u32 = stdin
        .lines()
        .map(|l| Card::new_from_line(&l.unwrap()))
        .map(|c| c.calc_point_value())
        .sum();

    println!("Total: {}", total);
}

#[derive(PartialEq, Eq, Debug)]
struct Card {
    winning_nums: HashSet<u32>,
    player_nums: HashSet<u32>,
}

impl Card {
    fn new_from_line(l: &str) -> Self {
        // Ex: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let nums: Vec<&str> = l.split(':').collect::<Vec<&str>>()[1].split('|').collect();

        let winning_nums = nums[0];
        let player_nums = nums[1];

        let split_n_parse = |nums: &str| -> HashSet<u32> {
            nums.split(' ')
                .map(|s| s.parse::<u32>().ok())
                .filter(|i| i.is_some())
                .map(|i| i.unwrap())
                .collect()
        };

        let winning_nums: HashSet<u32> = split_n_parse(winning_nums);
        let player_nums: HashSet<u32> = split_n_parse(player_nums);

        Self {
            winning_nums,
            player_nums,
        }
    }

    fn calc_point_value(&self) -> u32 {
        let mut val = 0;

        for pnum in &self.player_nums {
            if self.winning_nums.contains(&pnum) {
                match val {
                    0 => val = 1,
                    _ => val = val << 1,
                }
            }
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_new_from_line() {
        let test_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let test_card = Card::new_from_line(test_str);

        let expected_card = Card {
            winning_nums: HashSet::from([41, 48, 83, 86, 17]),
            player_nums: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        assert_eq!(test_card, expected_card);
    }

    #[test]
    fn test_card_cal_point_value() {
        let card = Card {
            winning_nums: HashSet::from([41, 48, 83, 86, 17]),
            player_nums: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        let expected_val = 8;

        assert_eq!(card.calc_point_value(), expected_val);
    }
}
