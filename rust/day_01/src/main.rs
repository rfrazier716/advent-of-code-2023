use std::fs;

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .map(|x| x.to_digit(10).unwrap()).collect();
            nums[0] * 10 + nums.last().unwrap()
        })
        .sum()
}

fn extract_first_and_last(input: &str) -> (u32, u32) {
    // Do two pointers where the tail is iterated over to see if a string exists. 
    let mut nums = Vec::new();
    let chars: Vec<_> = input.chars().collect();
    let mut tail = 0;
    for head in 0..input.len() {
        if chars[head].is_ascii_digit() {
            // if it's a digit push and reset the tail
            nums.push(chars[head].to_digit(10).unwrap());
            tail = head
        } else {
            for trailing in tail..head {
                if let Some(val) = match &input[trailing..=head] {
                    "one" => Some(1),
                    "two" => Some(2),
                    "three" => Some(3),
                    "four" => Some(4),
                    "five" => Some(5),
                    "six" => Some(6),
                    "seven" => Some(7),
                    "eight" => Some(8),
                    "nine" => Some(9),
                    _ => None,
                } {
                    nums.push(val);
                    tail = head
                }
            }
        }
    }
    (nums[0], *nums.last().unwrap())
}

fn part_two(input: &str) -> u32{
    input
        .lines()
        .map(|line| extract_first_and_last(line))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn main() {
    let input = fs::read_to_string("rust/day_01/input.txt").expect("Expected to load puzzle input");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extraction() {
        let act = extract_first_and_last("abcone2threexyz");
        assert_eq!(act, (1, 3));
    }
}
