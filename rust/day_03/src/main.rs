use itertools::iproduct;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Default)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn get_neighbors(&self, row_max: usize, col_max: usize) -> Vec<Coordinate> {
        iproduct!(-1..=1_isize, -1..=1_isize)
            .filter_map(
                |(row, col)| match (self.row as isize + row, self.column as isize + col) {
                    (-1, _) | (_, -1) => None,
                    (x, _) if x == row_max as isize => None,
                    (_, x) if x == col_max as isize => None,
                    (r, c) if r == self.row as isize && c == self.column as isize => None,
                    (row, column) => Some(Coordinate {
                        row: row as usize,
                        column: column as usize,
                    }),
                },
            )
            .collect()
    }
}

#[derive(Debug, Default)]
struct Schematic {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Schematic {
    fn get_symbol_coordinates(&self) -> Vec<Coordinate> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, character)| match character {
                '0'..='9' | '.' => None,
                _ => Some(Coordinate {
                    row: idx / self.cols,
                    column: idx % self.cols,
                }),
            })
            .collect()
    }

    fn get_gear_coordinates(&self) -> Vec<Coordinate> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, character)| {
                if *character == '*' {
                    Some(Coordinate {
                        row: idx / self.cols,
                        column: idx % self.cols,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_numbers(&self) -> Vec<(Coordinate, u32)> {
        let mut out: Vec<(Coordinate, u32)> = Vec::new();
        self.data.iter().enumerate().fold(0, |sum, (idx, val)| {
            let next = if let Some(num) = val.to_digit(10) {
                sum * 10 + num
            } else {
                0
            };
            // special case for line breaks
            if (idx + 1) % self.cols == 0 && next != 0 {
                out.push((
                    Coordinate {
                        row: idx / self.cols,
                        column: idx % self.cols,
                    },
                    next,
                ));
                return 0;
            } else if sum != 0 && next == 0 {
                out.push((
                    Coordinate {
                        row: idx / self.cols,
                        column: idx % self.cols,
                    },
                    sum,
                ));
            }
            next
        });
        out
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.data[row * self.cols + col]
    }

    fn to_idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

fn create_schematic(data: Vec<String>) -> Schematic {
    let mut iter = data.iter();
    let first = iter.next().expect("expected input");
    let cols = first.len();
    let data: Vec<_> = first
        .chars()
        .chain(iter.flat_map(|line| line.chars()))
        .collect();
    let rows = data.len() / cols;
    Schematic { data, rows, cols }
}

fn part_one(input: &Schematic) -> u32 {
    // get the list of coordinates we need to expand on
    let to_check: Vec<_> = input
        .get_symbol_coordinates()
        .into_iter()
        .flat_map(|coord| coord.get_neighbors(input.rows, input.cols))
        .filter(|neighbor| {
            let val = input.get(neighbor.row, neighbor.column);
            val.is_ascii_digit()
        })
        .collect();

    // get the list of numbers
    let nums: Vec<_> = input.get_numbers();
    let nums_idx: Vec<_> = nums
        .iter()
        .map(|(coord, _)| input.to_idx(coord.row, coord.column))
        .collect();

    // find the number related to the number to check
    let mut unique_idx: Vec<_> = to_check
        .into_iter()
        .map(|coord| input.to_idx(coord.row, coord.column))
        .map(|idx| nums_idx.partition_point(|&x| x <= idx))
        .collect::<Vec<_>>();
    unique_idx.sort();
    unique_idx.dedup();

    // finally -take the sum
    unique_idx.into_iter().map(|idx| nums[idx].1).sum()
}

fn part_two(input: &Schematic) -> u32 {
    // get the list of numbers
    let nums: Vec<_> = input.get_numbers();
    let nums_idx: Vec<_> = nums
        .iter()
        .map(|(coord, _)| input.to_idx(coord.row, coord.column))
        .collect();

    let gears = input.get_gear_coordinates();
    let mut sum = 0;
    for coord in gears {
        // get all the neighbors
        let neighbors: Vec<_> = coord
            .get_neighbors(input.rows, input.cols)
            .into_iter()
            .filter(|neighbor| {
                let val = input.get(neighbor.row, neighbor.column);
                val.is_ascii_digit()
            })
            .collect();

        // find the number related to the number to check
        let mut unique_idx: Vec<_> = neighbors
            .into_iter()
            .map(|coord| input.to_idx(coord.row, coord.column))
            .map(|idx| nums_idx.partition_point(|&x| x <= idx))
            .collect::<Vec<_>>();
        unique_idx.sort();
        unique_idx.dedup();

        if unique_idx.len() == 2{
            sum += nums[unique_idx[0]].1 * nums[unique_idx[1]].1
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("rust/day_03/input.txt").expect("Expected to load puzzle input");
    let sch = create_schematic(input.lines().map(|line| line.to_owned()).collect());
    println!("Part One Solution: {}", part_one(&sch));
    println!("Part Two Solution: {}", part_two(&sch));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let sch = create_schematic(input.lines().map(|line| line.to_owned()).collect());
        let soln = part_one(&sch);
        assert_eq!(soln, 4361)
    }
}
