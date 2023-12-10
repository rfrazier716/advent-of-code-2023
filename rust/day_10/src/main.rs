use std::collections::{HashSet, VecDeque};
use std::{fs, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PipeSegment {
    Vertical,   // a vertical bar
    Horizontal, // horizontal bar
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    PlusConnection, // can connect to any segment
}

impl FromStr for PipeSegment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PipeSegment::*;

        match s.as_bytes()[0] as char {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(BottomLeftCorner),
            'J' => Ok(BottomRightCorner),
            '7' => Ok(TopRightCorner),
            'F' => Ok(TopLeftCorner),
            'S' => Ok(PlusConnection),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for PipeSegment {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use PipeSegment::*;

        match value {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(BottomLeftCorner),
            'J' => Ok(BottomRightCorner),
            '7' => Ok(TopRightCorner),
            'F' => Ok(TopLeftCorner),
            'S' => Ok(PlusConnection),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]

pub struct PipeMap {
    map: Vec<Option<PipeSegment>>,
    rows: usize,
    cols: usize,
}

impl std::fmt::Display for PipeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in (0..self.rows) {
            let line: String = (0..self.cols)
                .map(|c| {
                    if let Some(pipe) = self.index(r, c) {
                        match pipe {
                            PipeSegment::Vertical => "|",
                            PipeSegment::Horizontal => "-",
                            PipeSegment::TopLeftCorner => "F",
                            PipeSegment::TopRightCorner => "7",
                            PipeSegment::BottomLeftCorner => "L",
                            PipeSegment::BottomRightCorner => "J",
                            PipeSegment::PlusConnection => "+",
                        }
                    } else {
                        "."
                    }
                })
                .collect();
            write!(f, "{}\n", line)?;
        }
        Ok(())
    }
}

impl PipeMap {
    pub fn find_start(&self) -> (usize, usize) {
        // since we replaced our start with a plus connection, search for that
        let idx = self
            .map
            .iter()
            .enumerate()
            .filter_map(|(i, element)| {
                if let Some(segment) = element {
                    if *segment == PipeSegment::PlusConnection {
                        Some(i)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .next()
            .expect("Pipe Start");
        (idx / self.cols, idx % self.cols)
    }

    pub fn index(&self, row: usize, col: usize) -> Option<PipeSegment> {
        // given an index returns the segment of pipe there
        if let Some(segment) = self.map.get(row * self.cols + col) {
            *segment
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: Option<PipeSegment>) {
        self.map[row * self.cols + col] = val;
    }

    pub fn connections(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        use PipeSegment::*;
        if let Some(segment) = self.index(row, col) {
            // check for above, below, left, right
            // doing module math to avoid an underflow
            let above = if row == 0 {
                None
            } else {
                match segment {
                    Vertical | BottomLeftCorner | BottomRightCorner | PlusConnection => {
                        let row_delta = (row + self.rows - 1) % self.rows;
                        if let Some(neighbor) = self.index(row_delta, col) {
                            match neighbor {
                                Vertical | TopLeftCorner | TopRightCorner | PlusConnection => {
                                    Some((row_delta, col))
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            };
            let below = if row == self.rows - 1 {
                None
            } else {
                match segment {
                    Vertical | TopLeftCorner | TopRightCorner | PlusConnection => {
                        let row_delta = row + 1;
                        if let Some(neighbor) = self.index(row_delta, col) {
                            match neighbor {
                                Vertical | BottomLeftCorner | BottomRightCorner
                                | PlusConnection => Some((row_delta, col)),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            };
            let left = if col == 0 {
                None
            } else {
                match segment {
                    Horizontal | TopRightCorner | BottomRightCorner | PlusConnection => {
                        let col_delta = (col + self.cols - 1) % self.cols;
                        if let Some(neighbor) = self.index(row, col_delta) {
                            match neighbor {
                                Horizontal | TopLeftCorner | BottomLeftCorner | PlusConnection => {
                                    Some((row, col_delta))
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            };
            let right = if col == self.cols - 1 {
                None
            } else {
                match segment {
                    Horizontal | TopLeftCorner | BottomLeftCorner | PlusConnection => {
                        let col_delta = col + 1;
                        if let Some(neighbor) = self.index(row, col_delta) {
                            match neighbor {
                                Horizontal | TopRightCorner | BottomRightCorner
                                | PlusConnection => Some((row, col_delta)),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            };
            // println!("{above:?}, {below:?}, {left:?}, {right:?}");
            [above, below, left, right]
                .into_iter()
                .filter_map(|neighbor| neighbor)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn traverse(&mut self) -> (usize, Vec<(usize, usize)>) {
        let start = self.find_start();
        let mut to_visit = VecDeque::from([start]);
        let mut visited = Vec::new();
        let mut distance = 0;
        // pull all the values off the deque and put any unvisited neighbors on
        while to_visit.len() != 0 {
            distance += 1;
            for _ in 0..to_visit.len() {
                // pop value off
                let node = to_visit.pop_front().expect("value");
                let neighbors = self.connections(node.0, node.1);
                self.set(node.0, node.1, None); // mark as visited
                visited.push(node);
                // println!("{neighbors:?}");
                for neighbor in neighbors.into_iter() {
                    to_visit.push_back(neighbor)
                }
            }
        }
        (distance - 1, visited)
    }
}

impl FromStr for PipeMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let cols = lines.peek().expect("line").len();
        let mapping: Vec<Option<PipeSegment>> = lines
            .flat_map(|line| line.chars())
            .map(|letter| letter.try_into().ok())
            .collect();
        let rows = mapping.len() / cols;
        Ok(PipeMap {
            map: mapping,
            rows,
            cols,
        })
    }
}

fn part_one(input: &str) -> usize {
    let mut map: PipeMap = input.parse().unwrap();
    map.traverse().0
}

fn part_two(input: &str) -> u32 {
    let map: PipeMap = input.parse().unwrap();
    let pipe_nodes = map.clone().traverse().1;
    // make a new 2d vec of u8's to use in our sum
    let mut mask: Vec<Vec<u32>> = vec![vec![0; map.cols]; map.rows];
    for (r, c) in pipe_nodes.iter() {
        // only count some nodes because of how we're doing our integral
        // this was guess and check, but its so that you can distinguish what is and isn't
        // in the polygon
        mask[*r][*c] = if let Some(segment) = map.index(*r, *c) {
            match segment {
                PipeSegment::Vertical
                | PipeSegment::BottomLeftCorner
                | PipeSegment::BottomRightCorner => 1,
                _ => 0,
            }
        } else {
            0
        }
    }

    let mut interior = 0;
    // now iterate over every node in the map and do a sum
    let pipe_nodes = HashSet::<(usize, usize)>::from_iter(pipe_nodes.into_iter());
    for r in 0..map.rows {
        for c in 0..map.cols {
            if !pipe_nodes.contains(&(r, c)) {
                let total: u32 = mask[r][0..c].iter().sum();
                if total % 2 == 1 {
                    // println!("Found interior cell at {r},{c}");
                    interior += 1
                }
            }
        }
    }

    interior
}

fn main() {
    let input = fs::read_to_string("rust/day_10/input.txt").expect("Expected to load puzzle input");

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_start() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let map: PipeMap = input.parse().expect("Expect Parse to Work");
        assert_eq!(map.cols, 5);
        assert_eq!(map.rows, 5);
        assert_eq!(map.find_start(), (2, 0));
    }

    #[test]
    fn test_find_connections() {
        let input = fs::read_to_string("test_input.txt").expect("Expected to load puzzle input");
        let map: PipeMap = input.parse().expect("Expect Parse to Work");
        let connections = map.connections(2, 0);
        println!("{connections:?}");
        assert_eq!(connections.len(), 2);
        assert!(connections.contains(&(3, 0)));
        assert!(connections.contains(&(2, 1)));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("test_input2.txt").expect("Expected to load puzzle input");
        let p2_soln = part_two(&input);
        println!("{p2_soln}");
        assert_eq!(p2_soln, 10);
    }
}
