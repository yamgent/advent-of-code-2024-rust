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

    fn add(&self, delta: (i64, i64), bounds: (usize, usize)) -> Option<Self> {
        let x = self.0 as i64 - delta.0;
        let y = self.1 as i64 - delta.1;

        if x < 0 || y < 0 || x >= bounds.0 as i64 || y >= bounds.1 as i64 {
            None
        } else {
            Some(Self(x as usize, y as usize))
        }
    }
}

struct Input {
    grid: Vec<Vec<char>>,
    bounds: (usize, usize),

    all_costs: HashMap<Pos, i64>,
    path: Vec<Pos>,
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
                .unwrap_or_else(|| panic!("{} should be in input", character))
        }
        let start = find_ch(&grid, 'S');
        let end = find_ch(&grid, 'E');

        grid[start.1][start.0] = '.';
        grid[end.1][end.0] = '.';

        let bounds = (grid[0].len(), grid.len());

        let mut all_costs = HashMap::new();
        let mut path = vec![];

        let mut next_cost = 0;
        let mut next_node = Some(start);

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
            .filter(|pos| grid[pos.1][pos.0] == '.')
            .find(|pos| !all_costs.contains_key(pos));
        }

        Self {
            grid,
            bounds,

            all_costs,
            path,
        }
    }
}

fn solve_p1(input: &str, limit: i64) -> String {
    let input = Input::parse_input(input);

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
                    .map(|pair_2| input.all_costs.get(&pair_2).expect("visited before"))
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

fn solve_p2(input: &str, limit: i64) -> String {
    let input = Input::parse_input(input);

    input
        .path
        .iter()
        .map(|start_cheat| {
            (-20i64..=20)
                .map(|dx| {
                    (-20i64..=20)
                        .filter(|dy| dx.abs() + dy.abs() <= 20)
                        .flat_map(|dy| start_cheat.add((dx, dy), input.bounds))
                        .filter(|end_cheat| input.grid[end_cheat.1][end_cheat.0] == '.')
                        .map(|end_cheat| {
                            input.all_costs.get(&end_cheat).expect("visited before")
                                - input.all_costs.get(start_cheat).expect("visited before")
                                - (start_cheat.0 as i64 - end_cheat.0 as i64).abs()
                                - (start_cheat.1 as i64 - end_cheat.1 as i64).abs()
                        })
                        .filter(|diff| *diff >= limit)
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    solve_p2(input, 100)
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
        let test_cases = [
            (14, 2),
            (14, 4),
            (2, 6),
            (4, 8),
            (2, 10),
            (3, 12),
            (1, 20),
            (1, 36),
            (1, 38),
            (1, 40),
            (1, 64),
        ];

        test_cases.iter().enumerate().for_each(|(case_id, case)| {
            let total_with = test_cases.iter().skip(case_id).map(|c| c.0).sum::<usize>();
            let total_without = test_cases
                .iter()
                .skip(case_id + 1)
                .map(|c| c.0)
                .sum::<usize>();

            assert_eq!(
                solve_p1(SAMPLE_INPUT, case.1 - 1),
                total_with.to_string(),
                "{} cheats, {} picoseconds test case: {} picoseconds expected {} total",
                case.0,
                case.1,
                case.1 - 1,
                total_with
            );

            assert_eq!(
                solve_p1(SAMPLE_INPUT, case.1),
                total_with.to_string(),
                "{} cheats, {} picoseconds test case: {} picoseconds expected {} total",
                case.0,
                case.1,
                case.1,
                total_with
            );
            assert_eq!(
                solve_p1(SAMPLE_INPUT, case.1 + 1),
                total_without.to_string(),
                "{} cheats, {} picoseconds test case: {} picoseconds expected {} total",
                case.0,
                case.1,
                case.1 + 1,
                total_with
            );
        });

        assert_eq!(p1(SAMPLE_INPUT), "0");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1338");
    }

    #[test]
    fn test_p2_sample() {
        let test_cases = [
            (32, 50),
            (31, 52),
            (29, 54),
            (39, 56),
            (25, 58),
            (23, 60),
            (20, 62),
            (19, 64),
            (12, 66),
            (14, 68),
            (12, 70),
            (22, 72),
            (4, 74),
            (3, 76),
        ];

        test_cases.iter().enumerate().for_each(|(case_id, case)| {
            let total_with = test_cases.iter().skip(case_id).map(|c| c.0).sum::<usize>();
            let total_without = test_cases
                .iter()
                .skip(case_id + 1)
                .map(|c| c.0)
                .sum::<usize>();

            assert_eq!(
                solve_p2(SAMPLE_INPUT, case.1 - 1),
                total_with.to_string(),
                "{} cheats, {} picoseconds test case: {} picoseconds expected {} total",
                case.0,
                case.1,
                case.1 - 1,
                total_with
            );

            assert_eq!(
                solve_p2(SAMPLE_INPUT, case.1),
                total_with.to_string(),
                "{} cheats, {} picoseconds test case: {} picoseconds expected {} total",
                case.0,
                case.1,
                case.1,
                total_with
            );
            assert_eq!(
                solve_p2(SAMPLE_INPUT, case.1 + 1),
                total_without.to_string(),
                "{} cheats, {} picoseconds test case: {} picoseconds expected {} total",
                case.0,
                case.1,
                case.1 + 1,
                total_with
            );
        });

        assert_eq!(p2(SAMPLE_INPUT), "0");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "975376");
    }
}
