use std::collections::{HashMap, HashSet};

use anyhow::{Result};
advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let num_matching = self.matching_numbers();
        if num_matching == 0 {
            return 0;
        }
        2_usize.pow((num_matching - 1) as u32) as u32
    }

    fn matching_numbers(&self) -> usize {
        let intersection = self.winning_numbers.intersection(&self.my_numbers);
        intersection.clone().count()
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|l| {
            let (_card, rest) = l.split_once(':').expect("card format");
            let (winning, all) = rest.split_once('|').expect("card format");

            let winning_numbers = winning
                .split_whitespace()
                .map(|d| d.parse::<u32>().expect("parse number"))
                .collect();

            let my_numbers = all
                .split_whitespace()
                .map(|d| d.parse::<u32>().expect("parse number"))
                .collect();
            Card {
                winning_numbers,
                my_numbers,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Result<u32> {
    let cards = parse_cards(input);
    Ok(cards.iter().fold(0, |acc, c| acc + c.points()))
}

pub fn part_two(input: &str) -> Result<u32> {
    let cards = parse_cards(input);

    let mut card_map = HashMap::new();
    for i in 0..cards.len() {
        card_map.insert(i, 1);
    }

    for (i, card) in cards.iter().enumerate() {
        let matching_numbers = card.matching_numbers();
        if matching_numbers == 0 {
            continue;
        }

        let num_current_cards = *card_map.get(&i).expect("card");

        let mut next_card = i + 1;
        while next_card <= cards.len().min(i + matching_numbers) {
            card_map.entry(next_card).and_modify(|e| {
                *e += num_current_cards;
            });
            next_card += 1;
        }
    }
    Ok(card_map.values().sum())
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
        assert_eq!(result.unwrap(), 13);
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
        assert_eq!(result.unwrap(), 30);
    }
}
