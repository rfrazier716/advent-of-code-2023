use std::fs;

fn part_one(input: &str) -> impl std::fmt::Display {
    0
}

fn part_two(input: &str) -> impl std::fmt::Display {
    0
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
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}