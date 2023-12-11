use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, str::FromStr};

struct SpaceMap {
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    planets: Vec<(usize, usize)>,
}

impl FromStr for SpaceMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let cols = lines.peek().unwrap().len();
        let rows = s.len() / cols;

        let mut empty_rows = HashSet::<usize>::from_iter(0..rows);
        let mut empty_cols = HashSet::<usize>::from_iter(0..cols);
        let mut planets: Vec<(usize, usize)> = Vec::new();

        for (r, line) in lines.enumerate() {
            for (c, element) in line.chars().enumerate() {
                match element {
                    '.' => {}
                    '#' => {
                        empty_rows.remove(&r);
                        empty_cols.remove(&c);
                        planets.push((r, c));
                    }
                    _ => unreachable!(),
                }
            }
        }

        Ok(Self {
            empty_rows: empty_rows.into_iter().collect(),
            empty_cols: empty_cols.into_iter().collect(),
            planets,
        })
    }
}

impl SpaceMap {
    fn distance_between(
        &self,
        (r1, c1): &(usize, usize),
        (r2, c2): &(usize, usize),
        expansion_coefficient: usize,
    ) -> usize {
        // do the manhattan distance plus addition for empty galaxies
        let dr = if r1 < r2 { r2 - r1 } else { r1 - r2 };
        let dc = if c1 < c2 { c2 - c1 } else { c1 - c2 };
        let manhattan_dist = dr + dc;
        let row_expansion = self
            .empty_rows
            .iter()
            .filter(|&row| *row > *r1.min(r2) && *row < *r1.max(r2))
            .count();
        let column_expansion = self
            .empty_cols
            .iter()
            .filter(|&col| *col > *c1.min(c2) && *col < *c1.max(c2))
            .count();

        manhattan_dist + (expansion_coefficient - 1) * (row_expansion + column_expansion)
    }

    fn min_spanning_distances(&self, expansion_coefficient: usize) -> usize {
        self.planets
            .iter()
            .combinations(2)
            .map(|comb| self.distance_between(comb[0], comb[1], expansion_coefficient))
            .sum()
    }
}

fn part_one(input: &str) -> usize {
    let map: SpaceMap = input.parse().expect("parsing");
    map.min_spanning_distances(2)
}

fn part_two(input: &str) -> usize {
    let map: SpaceMap = input.parse().expect("parsing");
    map.min_spanning_distances(1000000)
}

fn main() {
    let input = fs::read_to_string("rust/day_11/input.txt").expect("Expected to load puzzle input");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_galaxy_distance() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let map: SpaceMap = input.parse().expect("parsing");
        assert!(map.planets.contains(&(5, 1)));
        assert!(map.planets.contains(&(9, 4)));
        assert_eq!(map.distance_between(&(5, 1), &(9, 4), 1), 9)
    }

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let soln = part_one(&input);
        assert_eq!(soln, 374)
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let map: SpaceMap = input.parse().expect("parsing");
        let soln = map.min_spanning_distances(10);
        assert_eq!(soln, 1030)
    }
}
