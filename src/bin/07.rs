use anyhow::{bail, Error, Result};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]

struct Hand {
    r#type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(" ").expect("line format");

        let mut cards: [Card; 5] = [Card::Two; 5];

        for (i, c) in hand.chars().enumerate() {
            cards[i] = Card::from_char(c)?;
        }

        Ok(Hand {
            r#type: HandType::from_cards(cards),
            cards: cards,
            bid: bid.parse()?,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.r#type.cmp(&other.r#type);
        if ordering.is_ne() {
            return ordering;
        }

        for (i, card) in self.cards.iter().enumerate() {
            let card_ordering = card.cmp(&other.cards[i]);
            if card_ordering.is_ne() {
                return card_ordering;
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

impl Card {
    fn from_char(c: char) -> Result<Self> {
        let card = match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => bail!("invalid card character: {}", c),
        };
        Ok(card)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: [Card; 5]) -> Self {
        let mut set: HashMap<_, usize> = HashMap::new();
        for card in cards.iter() {
            *set.entry(card).or_insert(0) += 1;
        }

        if let Some(jokers) = set.get(&Card::J) {
            // can make five of a kind
            return match set.len() {
                1 | 2 => Self::FiveOfAKind,
                3 => {
                    // Q4J4J or AAA9J
                    if *jokers > 1 || *set.values().max().expect("5 cards") == 3 {
                        return Self::FourOfAKind;
                    }
                    // AAKKJ
                    Self::FullHouse
                }
                4 => Self::ThreeOfAKind,
                5 => Self::OnePair,
                _ => unreachable!(),
            };
        }

        match set.len() {
            1 => Self::FiveOfAKind,
            2 => match *set.values().max().expect("at least one card") {
                4 => Self::FourOfAKind,
                3 => Self::FullHouse,
                _ => unreachable!(),
            },
            3 => match *set.values().max().expect("at least one card") {
                3 => Self::ThreeOfAKind,
                2 => Self::TwoPair,
                _ => unreachable!(),
            },
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|l| Hand::from_str(l).expect("invalid hand"))
        .collect()
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut hands = parse_hands(input);
    hands.sort_by(|s, o| s.cmp(o));

    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        sum += (hands.len() - i) * h.bid;
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> Result<usize> {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            true,
            result.is_ok(),
            "part one failed: {}",
            result.err().unwrap()
        );
        assert_eq!(result.unwrap(), 6440);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            true,
            result.is_ok(),
            "part two failed: {}",
            result.err().unwrap()
        );
        assert_eq!(result.unwrap(), 5905);
    }
}
