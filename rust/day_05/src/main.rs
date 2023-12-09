use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;
use std::collections::VecDeque;
use std::{fs, str::FromStr};

#[derive(Parser)]
#[grammar = "parser.pest"]
struct PuzzleParser;

#[derive(Debug, Eq, PartialEq)]
struct Range {
    start: u64,
    span: u64,
}

impl Range {
    // fn map_onto(self, other: &Range) -> Vec<Range> {
    //     let end = self.start + self.span - 1;
    //     let other_end = other.start + other.span - 1;
    //     if end < other.start
    //         || self.start > other_end
    //         || self.start >= other.start && end <= other_end
    //     {
    //         vec![self]
    //     } else if self.start < other.start && end < other_end {
    //         vec![
    //             Range {
    //                 start: self.start,
    //                 span: other.start - self.start,
    //             },
    //             Range {
    //                 start: other.start,
    //                 span: end - other.start,
    //             },
    //         ]
    //     } else if self.start {
    //         vec![]
    //     }
    // }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MapElement {
    source_start: u64,
    target_start: u64,
    range: u64,
}

#[derive(Debug)]
struct Mapping(Vec<MapElement>);

impl Mapping {
    fn map(&self, source: u64) -> u64 {
        // assuming our list is sorted, find the point where this would be inserted, then check if the element falls in the range
        let partition = self.0.partition_point(|x| x.source_start < source);
        // if the partition index is zero, we don't transform this
        if partition == 0 {
            return source;
        }
        let potential_mapping = &self.0[partition - 1];
        let offset = source - potential_mapping.source_start;
        if potential_mapping.range > offset {
            potential_mapping.target_start + offset
        } else {
            source
        }
    }
    fn map_range(&self, source: Range) -> Vec<Range> {
        let mut mapped = Vec::new();
        let mut to_check = VecDeque::from([source]);

        // assuming our list is sorted, find the point where this would be inserted, then check if the element falls in the range
        while let Some(this_range) = to_check.pop_front() {
            let partition = self
                .0
                .partition_point(|x| x.source_start < this_range.start);
            println!("{this_range:?},{partition}");
            match partition {
                0 => {
                    if this_range.start + this_range.span - 1 < self.0[0].source_start {
                        // if our range ends before the first mapping start, push it untransformed
                        mapped.push(this_range)
                    } else {
                        // otherwise, split it, push the first part untransformed, and check the next part
                        let new_span = self.0[0].source_start - this_range.start;
                        mapped.push(Range {
                            start: this_range.start,
                            span: new_span,
                        });
                        to_check.push_back(Range {
                            start: self.0[0].source_start,
                            span: this_range.span - new_span,
                        });
                    }
                }
                x if x == self.0.len() => {
                    if self.0[x - 1].source_start + self.0[x - 1].range - 1 < this_range.start {
                        // if our range starts after the final mapping ends, push it untransformed
                        mapped.push(this_range)
                    } else {
                        // otherwise, split it, push the last part untransformed, and check the next part
                        let new_span =
                            self.0[x - 1].source_start + self.0[x - 1].range - this_range.start;
                        mapped.push(Range {
                            start: self.0[x - 1].source_start + self.0[x - 1].range,
                            span: this_range.span - new_span,
                        });
                        to_check.push_back(Range {
                            start: this_range.start,
                            span: new_span,
                        });
                    }
                }
                _ => {}
            }
        }

        mapped
    }
}

impl FromStr for MapElement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .split_whitespace()
            .map(|val| val.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .expect("expect map parsing to work");
        Ok(Self {
            source_start: vals[1],
            target_start: vals[0],
            range: vals[2],
        })
    }
}

struct PuzzleInput {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl FromStr for PuzzleInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file = PuzzleParser::parse(Rule::input, s)
            .expect("Expected Parse to work")
            .next()
            .unwrap();

        let mut seeds = vec![];
        let mut mappings: Vec<Mapping> = vec![];

        for element in file.into_inner() {
            match element.as_rule() {
                Rule::mapping => {
                    let mut map_elements = element
                        .into_inner()
                        .filter_map(|item| {
                            if item.as_rule() == Rule::mappingRow {
                                Some(item.as_str().parse::<MapElement>())
                            } else {
                                None
                            }
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .expect("expected mapping to parse");
                    map_elements.sort();
                    mappings.push(Mapping(map_elements));
                }
                Rule::seeds => {
                    seeds = element
                        .into_inner()
                        .filter_map(|x| {
                            if x.as_rule() == Rule::number {
                                Some(x.as_str().parse::<u64>())
                            } else {
                                None
                            }
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .expect("Expected Seed Parsing");
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        // part two hack for seeds
        seeds = seeds
            .chunks(2)
            .flat_map(|slice| slice[0]..(slice[0] + slice[1]))
            .collect();
        println!("{seeds:?}");

        Ok(Self { seeds, mappings })
    }
}

fn part_one(input: &str) -> u64 {
    let mut lowest_location = None;
    let input: PuzzleInput = input.parse().expect("Expected input to parse");
    for seed in input.seeds.iter() {
        let new_location = input
            .mappings
            .iter()
            .fold(*seed, |transform, mapping| mapping.map(transform));
        if let Some(current_low) = lowest_location {
            lowest_location = Some(std::cmp::min(current_low, new_location))
        } else {
            lowest_location = Some(new_location)
        }
    }
    lowest_location.unwrap()
}

fn part_two(input: &str) -> impl std::fmt::Display {
    0
}

fn main() {
    let input = fs::read_to_string("rust/day_05/input.txt").expect("Expected to load puzzle input");
    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_transform() {
        let mut elements = vec![
            MapElement {
                source_start: 50,
                target_start: 52,
                range: 48,
            },
            MapElement {
                source_start: 98,
                target_start: 50,
                range: 2,
            },
        ];
        elements.sort();
        let mapping = Mapping(elements);
        let test_cases: Vec<(u64, u64)> = vec![(79, 81), (14, 14), (55, 57), (13, 13), (100, 100)];
        for (input, exp) in test_cases.into_iter() {
            assert_eq!(exp, mapping.map(input));
        }
    }

    #[test]
    fn test_map_range_transform() {
        let mut elements = vec![
            MapElement {
                source_start: 50,
                target_start: 52,
                range: 48,
            },
            MapElement {
                source_start: 98,
                target_start: 50,
                range: 2,
            },
        ];
        elements.sort();
        let mapping = Mapping(elements);
        let test_cases: Vec<(Range, Vec<Range>)> = vec![
            (
                Range { start: 48, span: 2 },
                vec![Range { start: 48, span: 2 }],
            ),
            (
                Range {
                    start: 0,
                    span: 100,
                },
                vec![Range {
                    start: 100,
                    span: 2,
                }],
            ),
        ];
        for (input, exp) in test_cases.into_iter() {
            assert_eq!(exp, mapping.map_range(input));
        }
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1, 1);
    }
}
