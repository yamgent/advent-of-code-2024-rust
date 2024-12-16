use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::{HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/16/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(usize, usize, Direction);

impl Pos {
    fn advance(&self, bounds: (usize, usize)) -> Option<Self> {
        match self.2 {
            Direction::Up => self.1.checked_sub(1).map(|y| (self.0, y)),
            Direction::Down => {
                if self.1 + 1 >= bounds.1 {
                    None
                } else {
                    Some((self.0, self.1 + 1))
                }
            }
            Direction::Left => self.0.checked_sub(1).map(|x| (x, self.1)),
            Direction::Right => {
                if self.0 + 1 >= bounds.0 {
                    None
                } else {
                    Some((self.0 + 1, self.1))
                }
            }
        }
        .map(|coord| Self(coord.0, coord.1, self.2))
    }

    fn rotate_left(&self) -> Self {
        Self(
            self.0,
            self.1,
            match self.2 {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
        )
    }

    fn rotate_right(&self) -> Self {
        Self(
            self.0,
            self.1,
            match self.2 {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        )
    }
}

struct Input {
    start_pos: Pos,
    end_pos: Pos,
    grid: Vec<Vec<char>>,
}

fn find_known_character_in_grid(grid: &[Vec<char>], character: char, direction: Direction) -> Pos {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, cell)| {
                if *cell == character {
                    Some(Pos(x, y, direction))
                } else {
                    None
                }
            })
        })
        .unwrap_or_else(|| panic!("{} to exist in input", character))
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let mut grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let start_pos = find_known_character_in_grid(&grid, 'S', Direction::Right);
        let end_pos = find_known_character_in_grid(&grid, 'E', Direction::Right);

        grid[start_pos.1][start_pos.0] = '.';
        grid[end_pos.1][end_pos.0] = '.';

        Self {
            start_pos,
            end_pos,
            grid,
        }
    }

    fn simulate_p1(self) -> u64 {
        let bounds = (self.grid[0].len(), self.grid.len());
        let mut visited = HashSet::new();

        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        struct PathNode {
            cost: u64,
            pos: Pos,
        }

        let mut next_to_process = BinaryHeap::new();
        next_to_process.push(Reverse(PathNode {
            cost: 0,
            pos: self.start_pos,
        }));

        while let Some(next_node) = next_to_process.pop() {
            if visited.contains(&next_node.0.pos) {
                continue;
            }

            if next_node.0.pos.0 == self.end_pos.0 && next_node.0.pos.1 == self.end_pos.1 {
                return next_node.0.cost;
            }

            visited.insert(next_node.0.pos);

            fn consider_node(
                next_to_process: &mut BinaryHeap<Reverse<PathNode>>,
                visited: &HashSet<Pos>,
                grid: &[Vec<char>],
                pos: Option<Pos>,
                cost: u64,
            ) {
                if let Some(pos) = pos {
                    if grid[pos.1][pos.0] == '.' && !visited.contains(&pos) {
                        next_to_process.push(Reverse(PathNode { cost, pos }));
                    }
                }
            }

            consider_node(
                &mut next_to_process,
                &visited,
                &self.grid,
                next_node.0.pos.advance(bounds),
                next_node.0.cost + 1,
            );
            consider_node(
                &mut next_to_process,
                &visited,
                &self.grid,
                Some(next_node.0.pos.rotate_left()),
                next_node.0.cost + 1000,
            );
            consider_node(
                &mut next_to_process,
                &visited,
                &self.grid,
                Some(next_node.0.pos.rotate_right()),
                next_node.0.cost + 1000,
            );
        }

        /*
        {
            self.grid.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, ch)| {
                    print!(
                        "{}",
                        if [
                            Pos(x, y, Direction::Up),
                            Pos(x, y, Direction::Down),
                            Pos(x, y, Direction::Left),
                            Pos(x, y, Direction::Right),
                        ]
                        .into_iter()
                        .any(|pos| visited.contains(&pos))
                        {
                            '$'
                        } else {
                            *ch
                        }
                    );
                });
                println!();
            });
        }
        */

        panic!("cannot find a path, but input should always have a path");
    }
}

fn p1(input: &str) -> String {
    Input::parse_input(input).simulate_p1().to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
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
