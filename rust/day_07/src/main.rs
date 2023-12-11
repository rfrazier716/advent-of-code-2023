use std::collections::{HashMap, HashSet};
use std::fs;
use std::thread::current;

struct Hand(String);
struct HandWithWildcards(String);

enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn score(&self) -> u32 {
        // make a hashset for the characters
        let type_strength = self.get_type() as u32;
        let mut score: u32 = type_strength << 20;
        for (i, card) in self.0.chars().enumerate() {
            let card_strength = match card {
                '2'..='9' => card.to_digit(10).unwrap(),
                'T' => 0xA,
                'J' => 0xB,
                'Q' => 0xC,
                'K' => 0xD,
                'A' => 0xE,
                _ => unreachable!(),
            };
            score |= card_strength << 4 * (4 - i);
        }
        score
    }

    pub fn get_type(&self) -> HandType {
        use HandType::*;

        let mut card_counts = HashMap::<char, usize>::new();
        for letter in self.0.chars() {
            card_counts
                .entry(letter)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let unique_cards = card_counts.len();
        let max_pair = *card_counts.values().max().unwrap();
        match (unique_cards, max_pair) {
            (5, _) => HighCard,
            (4, _) => Pair,
            (3, 2) => TwoPair,
            (3, 3) => ThreeOfAKind,
            (2, 3) => FullHouse,
            (2, 4) => FourOfAKind,
            (1, _) => FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl HandWithWildcards {
    pub fn score(&self) -> u32 {
        // make a hashset for the characters
        let type_strength = self.get_type() as u32;
        let mut score: u32 = type_strength << 20;
        for (i, card) in self.0.chars().enumerate() {
            let card_strength = match card {
                '2'..='9' => card.to_digit(10).unwrap(),
                'T' => 0xA,
                'J' => 1,
                'Q' => 0xC,
                'K' => 0xD,
                'A' => 0xE,
                _ => unreachable!(),
            };
            score |= card_strength << 4 * (4 - i);
        }
        score
    }

    pub fn get_type(&self) -> HandType {
        use HandType::*;

        let mut card_counts = HashMap::<char, usize>::new();
        for letter in self.0.chars().filter(|c| *c != 'J') {
            card_counts
                .entry(letter)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let joker_count = self.0.chars().filter(|c| *c == 'J').count();

        let unique_cards = card_counts.len().max(1);
        let max_pair = if let Some(counts) = card_counts.values().max() {
            counts + joker_count
        } else {
            joker_count
        };
        match (unique_cards, max_pair) {
            (5, _) => HighCard,
            (4, _) => Pair,
            (3, 2) => TwoPair,
            (3, 3) => ThreeOfAKind,
            (2, 3) => FullHouse,
            (2, 4) => FourOfAKind,
            (1, _) => FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

fn part_one(input: &str) -> u32 {
    let mut wagers: Vec<_> = input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            (
                Hand(splits.next().expect("hand").into()),
                splits.next().expect("wager").parse::<u32>().expect("parse"),
            )
        })
        .collect();

    // sort by the score
    wagers.sort_by(|a, b| a.0.score().cmp(&b.0.score()));
    wagers
        .into_iter()
        .enumerate()
        .map(|(i, wager)| ((i as u32) + 1) * wager.1)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut wagers: Vec<_> = input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            (
                HandWithWildcards(splits.next().expect("hand").into()),
                splits.next().expect("wager").parse::<u32>().expect("parse"),
            )
        })
        .collect();

    // sort by the score
    wagers.sort_by(|a, b| a.0.score().cmp(&b.0.score()));
    wagers
        .into_iter()
        .enumerate()
        .map(|(i, wager)| ((i as u32) + 1) * wager.1)
        .sum()
}

fn main() {
    let input = fs::read_to_string("rust/day_07/input.txt").expect("Expected to load puzzle input");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        let hand = Hand("TJQKA".into());
        let cases = [("TJQKA", 0x000ABCDE), ("33222", 0x00433222)];

        for (input, expected) in cases.into_iter() {
            let hand = Hand(input.into());
            assert_eq!(hand.score(), expected)
        }
        println!("{:08X}", hand.score());
    }

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let soln = part_one(&input);
        assert_eq!(soln, 6440)
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let soln = part_two(&input);
        assert_eq!(soln, 5905)
    }
}
