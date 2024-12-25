use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/16/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn add(&self, delta: (i64, i64), bounds: (usize, usize)) -> Option<Self> {
        let x = (self.0 as i64) + delta.0;
        let y = (self.1 as i64) + delta.1;
        if x >= 0 && y >= 0 && x < (bounds.0 as i64) && y < (bounds.1 as i64) {
            Some(Pos(x as usize, y as usize))
        } else {
            None
        }
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let mut grid = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        fn find(grid: &[Vec<char>], character: char) -> Option<Pos> {
            grid.iter().enumerate().find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, ch)| **ch == character)
                    .map(|(x, _)| Pos(x, y))
            })
        }

        let start = find(&grid, 'S').expect("S in input");
        let end = find(&grid, 'E').expect("E in input");
        grid[start.1][start.0] = '.';
        grid[end.1][end.0] = '.';

        Self { grid, start, end }
    }

    fn bounds(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    fn get(&self, pos: Pos) -> Option<char> {
        self.grid
            .iter()
            .nth(pos.1)
            .and_then(|row| row.iter().nth(pos.0).copied())
    }

    fn go_dir(&self, cur_node: PathNode, new_dir: Direction) -> Option<PathNode> {
        let delta = match new_dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let additional_cost = if cur_node.direction == new_dir {
            0
        } else {
            match cur_node.direction {
                Direction::Up => match new_dir {
                    Direction::Up => 0,
                    Direction::Down => 2000,
                    Direction::Left => 1000,
                    Direction::Right => 1000,
                },
                Direction::Down => match new_dir {
                    Direction::Up => 2000,
                    Direction::Down => 0,
                    Direction::Left => 1000,
                    Direction::Right => 1000,
                },
                Direction::Left => match new_dir {
                    Direction::Up => 1000,
                    Direction::Down => 1000,
                    Direction::Left => 0,
                    Direction::Right => 2000,
                },
                Direction::Right => match new_dir {
                    Direction::Up => 1000,
                    Direction::Down => 1000,
                    Direction::Left => 2000,
                    Direction::Right => 0,
                },
            }
        };

        cur_node.pos.add(delta, self.bounds()).and_then(|pos| {
            if let Some(ch) = self.get(pos) {
                if ch == '.' {
                    Some(PathNode {
                        cost: cur_node.cost + 1 + additional_cost,
                        pos,
                        direction: new_dir,
                        prev_pos: cur_node.pos,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
struct MinHeap<T>(BinaryHeap<Reverse<T>>);

impl<T: std::cmp::Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self(iter.into_iter().map(|v| Reverse(v)).collect())
    }
}

impl<T: std::cmp::Ord> MinHeap<T> {
    fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|v| v.0)
    }

    fn push(&mut self, val: T) {
        self.0.push(Reverse(val));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PathNode {
    cost: usize,
    pos: Pos,
    direction: Direction,
    prev_pos: Pos,
}

#[derive(Debug)]
struct Parents {
    cost: usize,
    previous: HashSet<Pos>,
}

impl Default for Parents {
    fn default() -> Self {
        Self {
            cost: usize::MAX,
            previous: HashSet::new(),
        }
    }
}

impl Parents {
    fn register(&mut self, candidate: &PathNode) -> bool {
        match candidate.cost.cmp(&self.cost) {
            std::cmp::Ordering::Greater => false,
            std::cmp::Ordering::Equal => {
                self.previous.insert(candidate.prev_pos);
                true
            }
            std::cmp::Ordering::Less => {
                self.previous.clear();
                self.cost = candidate.cost;
                self.previous.insert(candidate.prev_pos);
                true
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Solution {
    Part1,
    Part2,
}

fn solve(input: &str, solution: Solution) -> String {
    let input = Map::parse_input(input);

    // TODO: can type be removed?
    let mut parents: HashMap<Pos, Parents> = HashMap::new();

    let mut to_process = [PathNode {
        cost: 0,
        pos: input.start,
        direction: Direction::Right,
        prev_pos: input.start,
    }]
    .into_iter()
    .collect::<MinHeap<_>>();

    while let Some(current_node) = to_process.pop() {
        if !parents
            .entry(current_node.pos)
            .or_default()
            .register(&current_node)
        {
            continue;
        }

        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .flat_map(|new_dir| input.go_dir(current_node, new_dir))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|path_node| {
            to_process.push(path_node);
        });
    }

    if solution == Solution::Part1 {
        parents
            .get(&input.end)
            .expect("end is not blocked")
            .cost
            .to_string()
    } else {
        "".to_string()
    }
}

fn p1(input: &str) -> String {
    solve(input, Solution::Part1)
}

fn p2(input: &str) -> String {
    solve(input, Solution::Part2)
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const SAMPLE_INPUT_2: &str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT_1), "7036");
        assert_eq!(p1(SAMPLE_INPUT_2), "11048");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "135536");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT_1), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
