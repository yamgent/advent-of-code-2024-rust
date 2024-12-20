use ahash::{HashMap, HashMapExt};
use itertools::Itertools;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/20/input.txt");

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pos(usize, usize);

impl Pos {
    fn up(&self) -> Option<Self> {
        self.1.checked_sub(1).map(|y| Self(self.0, y))
    }

    fn down(&self, bounds: (usize, usize)) -> Option<Self> {
        if self.1 + 1 < bounds.1 {
            Some(Self(self.0, self.1 + 1))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Self> {
        self.0.checked_sub(1).map(|x| Self(x, self.1))
    }

    fn right(&self, bounds: (usize, usize)) -> Option<Self> {
        if self.0 + 1 < bounds.0 {
            Some(Self(self.0 + 1, self.1))
        } else {
            None
        }
    }
}

struct Input {
    grid: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
    bounds: (usize, usize),
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let mut grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        fn find_ch(grid: &[Vec<char>], character: char) -> Pos {
            grid.iter()
                .enumerate()
                .find_map(|(y, row)| {
                    row.iter().enumerate().find_map(|(x, ch)| {
                        if *ch == character {
                            Some(Pos(x, y))
                        } else {
                            None
                        }
                    })
                })
                .expect(&format!("{} should be in input", character))
        }
        let start = find_ch(&grid, 'S');
        let end = find_ch(&grid, 'E');

        grid[start.1][start.0] = '.';
        grid[end.1][end.0] = '.';

        let bounds = (grid[0].len(), grid.len());

        Self {
            grid,
            start,
            end,
            bounds,
        }
    }
}

fn solve_p1(input: &str, limit: i32) -> String {
    let input = Input::parse_input(input);

    // TODO: path seems unused
    let (all_costs, _path) = {
        let mut all_costs = HashMap::new();
        let mut path = vec![];

        let mut next_cost = 0;
        let mut next_node = Some(input.start);

        while let Some(next) = next_node {
            all_costs.insert(next, next_cost);
            path.push(next);

            next_cost += 1;
            next_node = [
                Pos(next.0 - 1, next.1),
                Pos(next.0 + 1, next.1),
                Pos(next.0, next.1 - 1),
                Pos(next.0, next.1 + 1),
            ]
            .into_iter()
            .filter(|pos| input.grid[pos.1][pos.0] == '.')
            .filter(|pos| !all_costs.contains_key(&pos))
            .next();
        }

        (all_costs, path)
    };

    (0..input.bounds.1)
        .map(|y| {
            (0..input.bounds.0 - 1)
                .filter(|x| input.grid[y][*x] == '#')
                .filter(|x| {
                    let pair_1 = Pos(*x, y);

                    [
                        pair_1.left(),
                        pair_1.right(input.bounds),
                        pair_1.up(),
                        pair_1.down(input.bounds),
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|pair_2| input.grid[pair_2.1][pair_2.0] == '.')
                    .map(|pair_2| all_costs.get(&pair_2).expect("visited before"))
                    .permutations(2)
                    .map(|pairs| pairs[0] - pairs[1] - 2)
                    .filter(|diff| *diff >= limit)
                    .count()
                        > 0
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn p1(input: &str) -> String {
    solve_p1(input, 100)
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

    const SAMPLE_INPUT: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve_p1(SAMPLE_INPUT, 2), "44");
        assert_eq!(solve_p1(SAMPLE_INPUT, 3), "30");
        assert_eq!(solve_p1(SAMPLE_INPUT, 4), "30");
        assert_eq!(solve_p1(SAMPLE_INPUT, 5), "16");
        assert_eq!(solve_p1(SAMPLE_INPUT, 6), "16");
        assert_eq!(solve_p1(SAMPLE_INPUT, 7), "14");
        assert_eq!(solve_p1(SAMPLE_INPUT, 8), "14");
        assert_eq!(solve_p1(SAMPLE_INPUT, 9), "10");
        assert_eq!(solve_p1(SAMPLE_INPUT, 10), "10");
        assert_eq!(solve_p1(SAMPLE_INPUT, 11), "8");
        assert_eq!(solve_p1(SAMPLE_INPUT, 12), "8");
        assert_eq!(solve_p1(SAMPLE_INPUT, 13), "5");
        assert_eq!(solve_p1(SAMPLE_INPUT, 19), "5");
        assert_eq!(solve_p1(SAMPLE_INPUT, 20), "5");
        assert_eq!(solve_p1(SAMPLE_INPUT, 21), "4");
        assert_eq!(solve_p1(SAMPLE_INPUT, 35), "4");
        assert_eq!(solve_p1(SAMPLE_INPUT, 36), "4");
        assert_eq!(solve_p1(SAMPLE_INPUT, 37), "3");
        assert_eq!(solve_p1(SAMPLE_INPUT, 38), "3");
        assert_eq!(solve_p1(SAMPLE_INPUT, 39), "2");
        assert_eq!(solve_p1(SAMPLE_INPUT, 40), "2");
        assert_eq!(solve_p1(SAMPLE_INPUT, 41), "1");
        assert_eq!(solve_p1(SAMPLE_INPUT, 63), "1");
        assert_eq!(solve_p1(SAMPLE_INPUT, 64), "1");
        assert_eq!(solve_p1(SAMPLE_INPUT, 65), "0");
        assert_eq!(solve_p1(SAMPLE_INPUT, 66), "0");
        assert_eq!(p1(SAMPLE_INPUT), "0");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1338");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
