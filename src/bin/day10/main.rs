use ahash::{HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/10/input.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch == '.' {
                        999
                    } else {
                        ch.to_digit(10).expect("either a dot, or a digit")
                    }
                })
                .collect()
        })
        .collect()
}

fn p1(input: &str) -> String {
    let grid = parse_input(input);

    fn get_score(grid: &[Vec<u32>], x: usize, y: usize) -> usize {
        if grid[y][x] != 0 {
            0
        } else {
            let mut end = HashSet::new();

            fn visit(grid: &[Vec<u32>], x: usize, y: usize, end: &mut HashSet<(usize, usize)>) {
                let current_val = grid[y][x];
                let next_val = current_val + 1;

                if current_val == 9 {
                    end.insert((x, y));
                } else {
                    if x > 0 && grid[y][x - 1] == next_val {
                        visit(grid, x - 1, y, end);
                    }
                    if y > 0 && grid[y - 1][x] == next_val {
                        visit(grid, x, y - 1, end);
                    }
                    if x + 1 < grid[y].len() && grid[y][x + 1] == next_val {
                        visit(grid, x + 1, y, end);
                    }
                    if y + 1 < grid.len() && grid[y + 1][x] == next_val {
                        visit(grid, x, y + 1, end);
                    }
                }
            }

            visit(grid, x, y, &mut end);

            end.len()
        }
    }

    (0..grid.len())
        .into_iter()
        .map(|y| {
            (0..grid[y].len())
                .into_iter()
                .map(|x| get_score(&grid, x, y))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let grid = parse_input(input);

    fn get_rating(grid: &[Vec<u32>], x: usize, y: usize) -> usize {
        if grid[y][x] != 0 {
            0
        } else {
            let mut acc = 0;

            fn visit(grid: &[Vec<u32>], x: usize, y: usize, acc: &mut usize) {
                let current_val = grid[y][x];
                let next_val = current_val + 1;

                if current_val == 9 {
                    *acc += 1;
                } else {
                    if x > 0 && grid[y][x - 1] == next_val {
                        visit(grid, x - 1, y, acc);
                    }
                    if y > 0 && grid[y - 1][x] == next_val {
                        visit(grid, x, y - 1, acc);
                    }
                    if x + 1 < grid[y].len() && grid[y][x + 1] == next_val {
                        visit(grid, x + 1, y, acc);
                    }
                    if y + 1 < grid.len() && grid[y + 1][x] == next_val {
                        visit(grid, x, y + 1, acc);
                    }
                }
            }

            visit(grid, x, y, &mut acc);
            acc
        }
    }

    (0..grid.len())
        .into_iter()
        .map(|y| {
            (0..grid[y].len())
                .into_iter()
                .map(|x| get_rating(&grid, x, y))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1(r"
0123
1234
8765
9876
"),
            "1"
        );
        assert_eq!(
            p1(r"
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
"),
            "2"
        );
        assert_eq!(
            p1(r"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"),
            "4"
        );
        assert_eq!(
            p1(r"
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
"),
            "3"
        );
        assert_eq!(
            p1(r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"),
            "36"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "552");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2(r"
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
"),
            "3"
        );
        assert_eq!(
            p2(r"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"),
            "13"
        );
        assert_eq!(
            p2(r"
012345
123456
234567
345678
4.6789
56789.
"),
            "227"
        );
        assert_eq!(
            p2(r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"),
            "81"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1225");
    }
}
