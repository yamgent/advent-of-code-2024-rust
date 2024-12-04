const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/04/input.txt");

#[derive(Clone, Copy)]
struct Coord(i64, i64);

impl Coord {
    fn left(self) -> Self {
        Self(self.0 - 1, self.1)
    }

    fn right(self) -> Self {
        Self(self.0 + 1, self.1)
    }

    fn up(self) -> Self {
        Self(self.0, self.1 - 1)
    }

    fn down(self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn upleft(self) -> Self {
        Self(self.0 - 1, self.1 - 1)
    }

    fn upright(self) -> Self {
        Self(self.0 + 1, self.1 - 1)
    }

    fn downleft(self) -> Self {
        Self(self.0 - 1, self.1 + 1)
    }

    fn downright(self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }
}

fn p1(input: &str) -> String {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn get(grid: &Vec<Vec<char>>, pos: Coord) -> char {
        if pos.0 >= 0 && pos.0 < grid.len() as i64 && pos.1 >= 0 && pos.1 < grid[0].len() as i64 {
            grid[pos.0 as usize][pos.1 as usize]
        } else {
            ' '
        }
    }

    (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .map(|x| {
                    let coord = Coord(x as i64, y as i64);
                    [
                        Coord::left,
                        Coord::right,
                        Coord::up,
                        Coord::down,
                        Coord::upleft,
                        Coord::upright,
                        Coord::downleft,
                        Coord::downright,
                    ]
                    .map(|dir| {
                        (0..4)
                            .fold((vec![], coord), |(mut acc, coord), _| {
                                acc.push(get(&grid, coord));
                                (acc, dir(coord))
                            })
                            .0
                            .into_iter()
                            .collect::<String>()
                            == "XMAS"
                    })
                    .into_iter()
                    .filter(|x| *x)
                    .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn get(grid: &Vec<Vec<char>>, pos: Coord) -> char {
        if pos.0 >= 0 && pos.0 < grid.len() as i64 && pos.1 >= 0 && pos.1 < grid[0].len() as i64 {
            grid[pos.0 as usize][pos.1 as usize]
        } else {
            ' '
        }
    }

    (1..grid.len() - 1)
        .map(|y| {
            (1..grid[y].len() - 1)
                .filter(|x| {
                    let coord = Coord(*x as i64, y as i64);

                    if get(&grid, coord) == 'A' {
                        let collect = [
                            coord.upleft(),
                            coord.upright(),
                            coord.downright(),
                            coord.downleft(),
                        ]
                        .into_iter()
                        .map(|coord| get(&grid, coord))
                        .collect::<String>();
                        ["MMSS", "SMMS", "SSMM", "MSSM"]
                            .into_iter()
                            .any(|entry| entry == collect)
                    } else {
                        false
                    }
                })
                .count()
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

    const SAMPLE_INPUT: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    ";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "18");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "2454");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "9");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1858");
    }
}
