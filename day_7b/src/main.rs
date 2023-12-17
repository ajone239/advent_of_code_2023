use std::{cmp::Ordering, collections::HashMap, io};

fn main() {
    let stdin = io::stdin();

    let mut players: Vec<Player> = stdin.lines().map(|l| Player::new(&l.unwrap())).collect();

    players.sort_by(|a, b| a.hand.cmp(&b.hand));

    let total: u64 = players
        .into_iter()
        .enumerate()
        .map(|(i, p)| p.bid * (i as u64 + 1))
        .sum();

    println!("Total: {}", total);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    bid: u64,
    hand: Hand,
}

impl Player {
    fn new(s: &str) -> Self {
        let fields: Vec<&str> = s.split(' ').collect();

        let hand = fields[0];
        let bid = fields[1];

        let bid: u64 = bid.parse().unwrap();

        let hand = Hand::new(hand);

        Self { bid, hand }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    hand_str: String,
    hand: [Card; 5],
    rank: HandRank,
}

impl Hand {
    fn new(s: &str) -> Self {
        let mut hand = [Card::Num(0); 5];

        for (i, c) in s.chars().enumerate() {
            hand[i] = Card::new(c);
        }

        let rank = HandRank::new(&hand);

        Self {
            hand_str: s.to_owned(),
            hand,
            rank,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.hand.cmp(&other.hand),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    fn new(hand: &[Card; 5]) -> HandRank {
        let mut card_counts = HashMap::new();
        let mut joker_count = 0;

        for card in hand {
            if *card == Card::Joker {
                joker_count += 1;
                continue;
            }
            let card_count = card_counts.entry(card).or_insert(0);
            *card_count += 1;
        }

        let mut card_counts: Vec<u64> = card_counts.iter().map(|(_, count)| *count).collect();

        if card_counts.is_empty() {
            card_counts.push(joker_count)
        } else {
            card_counts.sort_by(|a, b| b.cmp(&a));
            card_counts[0] += joker_count;
        }

        let mut card_counts = card_counts.into_iter();

        match card_counts.next().unwrap() {
            5 => HandRank::FiveOfAKind,
            4 => HandRank::FourOfAKind,
            3 => match card_counts.next().unwrap() {
                2 => HandRank::FullHouse,
                1 => HandRank::ThreeOfAKind,
                _ => unreachable!(),
            },
            2 => match card_counts.next().unwrap() {
                2 => HandRank::TwoPair,
                1 => HandRank::OnePair,
                _ => unreachable!(),
            },
            1 => HandRank::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Num(u8),
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Num(10),
            '0'..='9' => Card::Num(c as u8 - '0' as u8),
            _ => unreachable!(),
        }
    }
}
