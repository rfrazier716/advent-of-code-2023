use std::collections::{HashSet, VecDeque};
use std::fs;

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut nums = line.split(':').last().unwrap().split('|').map(|nums| {
        nums.split(' ')
            .filter_map(|val| if val.is_empty() { None } else { Some(val) })
            .map(|val| val.parse::<u32>().expect("Expected to parse num"))
            .collect()
    });
    (nums.next().unwrap(), nums.next().unwrap())
}

fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for (winning, yours) in input.lines().map(parse_line) {
        let winning_set = HashSet::<u32>::from_iter(winning);
        let winning_numbers: Vec<_> = yours
            .iter()
            .filter(|num| winning_set.contains(num))
            .collect();
        let win_count = winning_numbers.len();
        sum += if win_count > 0 {
            2_u32.pow(win_count as u32 - 1)
        } else {
            0
        }
    }
    sum
}

fn part_two(input: &str) -> u32 {
    let winnings: Vec<_> = input
        .lines()
        .map(parse_line)
        .map(|(winning, yours)| {
            let winning_set = HashSet::<u32>::from_iter(winning);
            let winning_numbers: Vec<_> = yours
                .iter()
                .filter(|num| winning_set.contains(num))
                .collect();
            winning_numbers.len()
        })
        .rev()
        .collect();

    let mut total_count_per_card: Vec<u32> = vec![0; winnings.len()];

    for (i, winning) in winnings.iter().enumerate() {
        total_count_per_card[i] = total_count_per_card[i - winning..i].iter().sum::<u32>() + 1
    }

    total_count_per_card.into_iter().sum()
}

fn main() {
    let input = fs::read_to_string("rust/day_04/input.txt").expect("Expected to load puzzle input");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
