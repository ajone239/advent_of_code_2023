use std::{collections::HashSet, io};

fn main() {
    let stdin = io::stdin();

    let cards: Vec<Card> = stdin
        .lines()
        .map(|l| Card::new_from_line(&l.unwrap()))
        .collect();

    let total = expand_card_copies_2(cards);

    println!("Total: {}", total);
}

fn expand_card_copies_2(mut cards: Vec<Card>) -> u32 {
    for i in 0..cards.len() {
        let points = cards[i].count_matches();

        for j in 0..points {
            let card_id: usize = i + j as usize + 1;
            if card_id > cards.len() {
                break;
            }
            cards[card_id].copies += cards[i].copies;
        }
    }
    cards.into_iter().map(|c| c.copies).sum()
}

fn expand_card_copies(mut cards: Vec<Card>) -> u32 {
    let mut starting_point = 0;
    let init_len = cards.len();
    loop {
        let curr_len = cards.len();
        println!("here: {}: {}: {}", init_len, curr_len, starting_point);
        for i in starting_point..curr_len {
            let points = cards[i].count_matches();

            for j in 0..points {
                let card_id: usize = (cards[i].id + j + 1) as usize;
                if card_id > init_len {
                    break;
                }
                cards.push(cards[card_id - 1].clone());
            }
        }

        if cards.len() == curr_len {
            break;
        }

        starting_point = curr_len;
    }

    return cards.len() as u32;
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Card {
    id: u32,
    copies: u32,
    winning_nums: HashSet<u32>,
    player_nums: HashSet<u32>,
}

impl Card {
    fn new_from_line(l: &str) -> Self {
        // Ex: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let card_details: Vec<&str> = l.split(':').collect();

        let card_id = card_details[0];
        let id: u32 = card_id.split(' ').last().unwrap().parse().unwrap();

        let nums = card_details[1];
        let nums: Vec<&str> = nums.split('|').collect();

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
            id,
            copies: 1,
            winning_nums,
            player_nums,
        }
    }

    fn count_matches(&self) -> u32 {
        let mut val = 0;

        for pnum in &self.player_nums {
            if self.winning_nums.contains(&pnum) {
                val += 1;
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
            id: 1,
            copies: 1,
            winning_nums: HashSet::from([41, 48, 83, 86, 17]),
            player_nums: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        assert_eq!(test_card, expected_card);
    }

    #[test]
    fn test_card_cal_point_value() {
        let card = Card {
            id: 1,
            copies: 1,
            winning_nums: HashSet::from([41, 48, 83, 86, 17]),
            player_nums: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        let expected_val = 4;

        assert_eq!(card.count_matches(), expected_val);
    }
}
