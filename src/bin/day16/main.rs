use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

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
        self.grid.get(pos.1).and_then(|row| row.get(pos.0).copied())
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
                        prev_dir: cur_node.direction,
                        prev_cost: cur_node.cost,
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
    prev_dir: Direction,
    prev_cost: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ParentPrevious {
    current_direction: Direction,
    current_pos: Pos,
    parent_direction: Direction,
    parent_pos: Pos,
    reach_cost: usize,
    parent_cost: usize,
}

#[derive(Debug)]
struct Parents {
    minimum_cost: usize,
    previous: HashMap<(Pos, Direction), ParentPrevious>,
}

impl Default for Parents {
    fn default() -> Self {
        Self {
            minimum_cost: usize::MAX,
            previous: HashMap::new(),
        }
    }
}

impl Parents {
    fn register(&mut self, candidate: &PathNode) -> bool {
        self.minimum_cost = self.minimum_cost.min(candidate.cost);

        let key = (candidate.prev_pos, candidate.prev_dir);
        let seen_before = self.previous.contains_key(&key);

        if !seen_before {
            self.previous.insert(
                key,
                ParentPrevious {
                    current_direction: candidate.direction,
                    current_pos: candidate.pos,
                    parent_direction: candidate.prev_dir,
                    parent_pos: candidate.prev_pos,
                    reach_cost: candidate.cost,
                    parent_cost: candidate.prev_cost,
                },
            );
        }

        !seen_before
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
        prev_dir: Direction::Right,
        prev_cost: 0,
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
    let end_cost = parents
        .get(&input.end)
        .expect("end is not blocked")
        .minimum_cost;

    if solution == Solution::Part1 {
        end_cost.to_string()
    } else {
        //let mut visited = [(input.end)].into_iter().collect::<HashSet<_>>();

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct ToProcess {
            pos: Pos,
            //direction: Direction,
            cost: usize,
        }

        //let mut to_process = parents
        //    .get(&input.end)
        //    .expect("visited before")
        //    .previous
        //    .values()
        //    .filter(|val| val.reach_cost == end_cost)
        //    .map(|val| ToProcess {
        //        pos: val.parent_pos,
        //        //direction: val.parent_direction,
        //        cost: val.parent_cost,
        //    })
        //    .collect::<VecDeque<_>>();
        //
        let mut visited = HashSet::new();
        let mut to_process = [ToProcess {
            pos: input.end,
            cost: end_cost,
        }]
        .into_iter()
        .collect::<VecDeque<_>>();

        while let Some(next_to_process) = to_process.pop_front() {
            visited.insert(next_to_process.pos);

            if next_to_process.pos == input.start {
                continue;
            }

            parents
                .get(&next_to_process.pos)
                .expect("visited before")
                .previous
                .values()
                .filter(|val| val.reach_cost == next_to_process.cost)
                .map(|val| ToProcess {
                    pos: val.parent_pos,
                    //direction: val.parent_direction,
                    cost: val.parent_cost,
                })
                .for_each(|val| {
                    to_process.push_back(val);
                });
        }

        visited.len().to_string()
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
        // shortest path is 2006
        // at (3, 1), there are two paths.
        //      - 1 is facing >, cost 2005 [still a valid path, even if not min at this point]
        //      - 1 is facing ^, cost 1005
        assert_eq!(
            p2(r"
######
#...E#
#.#.##
#.#.##
#S..##
######
"),
            "11"
        );
        assert_eq!(p2(SAMPLE_INPUT_1), "45");
        assert_eq!(p2(SAMPLE_INPUT_2), "64");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "583");
    }
}
