use std::{collections::HashMap, io};

fn main() {
    let stdin = io::stdin();

    let players: Vec<Player> = stdin.lines().map(|l| Player::new(&l.unwrap())).collect();

    for p in players {
        println!("{:?}", p);
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Hand {
    hand_str: String,
    hand: [Card; 5],
}

impl Hand {
    fn new(s: &str) -> Self {
        let mut hand = [Card::Num(0); 5];

        for (i, c) in s.chars().enumerate() {
            hand[i] = Card::new(c);
        }

        Self {
            hand_str: s.to_owned(),
            hand,
        }
    }

    fn rank(&self) -> HandRank {
        let mut hand = HashMap::new();

        for card in self.hand {
            let card_count = hand.entry(card).or_insert(0);
            *card_count += 1;
        }

        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug)]
enum HandRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Num(u8),
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Num(10),
            '0'..='9' => Card::Num(c as u8 - '0' as u8),
            _ => unreachable!(),
        }
    }
}
