use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::{fs, str::FromStr};

#[derive(Default, Debug)]
struct Drawing {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Drawing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // create our regex
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\s*(?P<amount>[0-9]+)\s(?P<color>red|green|blue)\s*$").unwrap();
        }

        // split at ',' and get the amount and color
        let mut drawing = Self::default();
        for capture in s.split(',').filter_map(|to_match| RE.captures(to_match)) {
            let count = capture
                .name("amount")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .map_err(|_| "Couldn't parse amount")?;
            match capture.name("color").unwrap().as_str() {
                "red" => drawing.red += count,
                "blue" => drawing.blue += count,
                "green" => drawing.green += count,
                _ => return Err("Couldn't Match color".into()),
            }
        }

        Ok(drawing)
    }
}

#[derive(Debug)]
struct Round(Vec<Drawing>);

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let drawings = s.split(':').last().unwrap().split(';');
        let typed = drawings
            .map(|drawing| Drawing::from_str(drawing))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Round(typed))
    }
}

fn minimum_cube_count(round: &Round) -> Drawing {
    round
        .0
        .iter()
        .fold(Drawing::default(), |min_drawing, current_drawing| Drawing {
            red: max(min_drawing.red, current_drawing.red),
            green: max(min_drawing.green, current_drawing.green),
            blue: max(min_drawing.blue, current_drawing.blue),
        })
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| Round::from_str(line).expect("could not parse line"))
        .map(|round| minimum_cube_count(&round))
        .enumerate()
        .filter_map(|(i, count)| {
            if count.red <= 12 && count.green <= 13 && count.blue <= 14 {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Round::from_str(line).expect("could not parse line"))
        .map(|round| minimum_cube_count(&round))
        .map(|count| count.red * count.blue * count.green)
        .sum()
}

fn main() {
    let input = fs::read_to_string("rust/day_02/input.txt").expect("Expected to load puzzle input");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cube_count() {
        let round = Round::from_str("Game 8: 2 blue, 12 red; 1 green, 2 blue, 10 red; 12 red, 10 blue; 5 red, 1 green, 2 blue; 13 red, 16 blue, 1 green; 2 blue, 18 red").unwrap();
        let min = minimum_cube_count(&round);
        assert_eq!(min.red, 18);
    }
}
