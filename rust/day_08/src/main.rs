use itertools::{Itertools, Tuples};
use pest::Parser;
use pest_derive::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, str::FromStr};

#[derive(Parser)]
#[grammar = "parser.pest"]
struct PuzzleParser;

type NodeId = String;

#[derive(Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

#[derive(Debug)]
pub struct Cycle {
    exits: HashSet<usize>,
    size: usize,
}

impl Cycle {
    pub fn intersect(self, other: Cycle) -> Self {
        let lcm = num::integer::lcm(self.size, other.size);
        let extended_exits = HashSet::<_>::from_iter(
            self.exits
                .into_iter()
                .flat_map(|val| (0..(lcm / self.size)).map(move |x| x * self.size + val)),
        );
        let other_extended_exits = HashSet::from_iter(
            other
                .exits
                .into_iter()
                .flat_map(|val| (0..(lcm / other.size)).map(move |x| x * other.size + val)),
        );
        let intersection =
            HashSet::from_iter(extended_exits.intersection(&other_extended_exits).copied());
        Self {
            exits: intersection,
            size: lcm,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    route: String,
    mapping: HashMap<String, Node>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = PuzzleParser::parse(Rule::input, s)
            .expect("Expected to Parse")
            .next()
            .unwrap();
        let mut pairs = parsed.into_inner();
        let route = pairs.next().unwrap().as_str().to_owned();
        let mappings = pairs
            .filter_map(|item| {
                if item.as_rule() == Rule::mapEntry {
                    Some(item)
                } else {
                    None
                }
            })
            .map(|entries| {
                let mut elements = entries.into_inner();
                let source = elements.next().unwrap().as_str().to_owned();
                let left = elements.next().unwrap().as_str().to_owned();
                let right = elements.next().unwrap().as_str().to_owned();
                (source, Node { left, right })
            });
        let mapping = HashMap::from_iter(mappings);
        Ok(Self { route, mapping })
    }
}

impl Input {
    pub fn cycle(&self, from: &str) -> String {
        // go through a full round of input
        self.route
            .chars()
            .fold(&from.into(), |current, direction| {
                let paths = self
                    .mapping
                    .get(current)
                    .expect("Expected Map Key to Exist");
                if direction == 'L' {
                    &paths.left
                } else {
                    &paths.right
                }
            })
            .into()
    }

    pub fn find_cycle(&self, start: &str) -> Cycle {
        let mut seen_starts: HashMap<String, usize> = HashMap::new();
        let mut start: String = start.into();
        let mut iterations = 0;
        while seen_starts.get(&start) == None {
            let next = self.cycle(&start);
            seen_starts.insert(start, iterations);
            iterations += 1;
            start = next;
        }
        let cycle_start = seen_starts.get(&start).unwrap();
        let cycle_length = (iterations - cycle_start) * self.route.len();
        let exits = HashSet::from_iter(
            self.find_exits(&start, cycle_length)
                .into_iter()
                .map(|x| x + cycle_start * self.route.len()),
        );
        Cycle {
            exits,
            size: (iterations - cycle_start) * self.route.len(),
        }
    }

    pub fn find_exits(&self, start: &str, max_iterations: usize) -> Vec<usize> {
        let mut exits = Vec::new();
        let mut node: &String = &start.into();
        for step in 0..max_iterations {
            if node.ends_with('Z') {
                exits.push(step);
            }
            let direction = self.route.as_bytes()[step % self.route.len()] as char;
            let path = self.mapping.get(node).expect("node in map");
            node = if direction == 'L' {
                &path.left
            } else {
                &path.right
            };
        }
        exits
    }
}

fn part_one(input: &Input) -> impl std::fmt::Display {
    let mut iterations = 0;
    let mut next: Option<&str> = Some("AAA");
    while let Some(node) = next {
        let direction = input.route.as_bytes()[iterations % input.route.len()] as char;
        let paths = input.mapping.get(node).unwrap();
        let next_node = if direction == 'L' {
            &paths.left
        } else {
            &paths.right
        };
        next = if next_node == "ZZZ" {
            None
        } else {
            Some(&next_node)
        };
        iterations += 1;
    }
    iterations
}

fn part_two(input: &Input) -> usize {
    // kind of annoyed with part two. I thought we would have to do a more general merging of multiple cycles
    // to find the points where they both exit at the same time

    // turns out each cycle has one exit, and the exit is the cycle length, so you just take the LCM of them all...
    let cycles_iter = input
        .mapping
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|start| input.find_cycle(start));

    let cycles: Vec<_> = cycles_iter.collect();
    // println!("{:?}", cycles);
    cycles.into_iter().fold(1, |lcm, cycle| num::integer::lcm(lcm, cycle.size))

    // This is the code that would have done it the other way but it chugs!
    // need to find a better way to calculate the intersections
    // let final_cycle = cycles_iter.fold(None, |current: Option<Cycle>, next| {
    //     println!("{current:?}");
    //     if let Some(current_cycle) = current {
    //         Some(current_cycle.intersect(next))
    //     } else {
    //         Some(next)
    //     }
    // }).unwrap();

    // final_cycle.exits.into_iter().min().unwrap()
}

fn main() {
    let input_str =
        fs::read_to_string("rust/day_08/input.txt").expect("Expected to load puzzle input");
    let input = Input::from_str(&input_str).expect("Expected to parse");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cycles() {
        let input_str =
            fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let input = Input::from_str(&input_str).expect("Expected to parse");
        let cycle = input.find_cycle("22A");
        assert_eq!(cycle.size, 6);
        assert!(cycle.exits.contains(&3));
        assert!(cycle.exits.contains(&6));
    }

    #[test]
    fn test_cycle_intersect() {
        let c1 = Cycle {
            exits: HashSet::from([2]),
            size: 2,
        };
        let c2 = Cycle {
            exits: HashSet::from([3, 6]),
            size: 6,
        };
        let merged = c1.intersect(c2);
        assert_eq!(merged.size, 6);
        assert!(merged.exits.contains(&6));
        assert_eq!(merged.exits.len(), 1);
    }

    #[test]
    fn test_part_two() {
        let input_str =
            fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let input = Input::from_str(&input_str).expect("Expected to parse");
        assert_eq!(part_two(&input), 6);
    }
}
