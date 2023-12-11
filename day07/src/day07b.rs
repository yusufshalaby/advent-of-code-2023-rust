use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
enum Rank {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn new(rank: char) -> Rank {
        match rank {
            'J' => Rank::Joker,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard(Vec<Rank>),
    OnePair(Vec<Rank>),
    TwoPair(Vec<Rank>),
    ThreeOfAKind(Vec<Rank>),
    FullHouse(Vec<Rank>),
    FourOfAKind(Vec<Rank>),
    FiveOfAKind(Vec<Rank>),
}

impl HandType {
    fn new(hand: &str) -> HandType {
        let ranks = hand.chars().map(|c| Rank::new(c)).collect::<Vec<_>>();
        let mut counts = HashMap::new();
        for rank in &ranks {
            *counts.entry(rank).or_insert(0) += 1;
        }
        let max_non_joker = *counts
            .iter()
            .filter(|(&key, _)| key != &Rank::Joker)
            .max_by_key(|(_, &count)| count)
            .unwrap_or((&&Rank::Joker, &0))
            .1;

        let num_joker = counts.get(&Rank::Joker).unwrap_or(&0);
        match max_non_joker + num_joker {
            5 => HandType::FiveOfAKind(ranks),
            4 => HandType::FourOfAKind(ranks),
            3 => {
                if counts.len() == 2 || (counts.len() == 3 && num_joker == &1) {
                    HandType::FullHouse(ranks)
                } else {
                    HandType::ThreeOfAKind(ranks)
                }
            }
            2 => {
                if counts.len() == 3 || (counts.len() == 4 && num_joker == &1) {
                    HandType::TwoPair(ranks)
                } else {
                    HandType::OnePair(ranks)
                }
            }
            1 => HandType::HighCard(ranks),
            _ => panic!("Invalid hand: {}", hand),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    bid: i32,
}

pub fn day07b(input: &str) -> i32 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand = HandType::new(hand);
            Hand {
                hand_type: hand,
                bid: bid.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| b.hand_type.cmp(&a.hand_type));
    hands
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as i32 + 1))
}
