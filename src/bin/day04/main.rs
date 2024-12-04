const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/04/input.txt");

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        Self {
            cells: input
                .trim()
                .lines()
                .map(|line| line.trim().chars().collect())
                .collect(),
        }
    }

    fn get(&self, pos: Coord) -> char {
        const INVALID: char = ' ';

        if pos.0 >= 0 && pos.1 >= 0 {
            let x = pos.0 as usize;
            let y = pos.1 as usize;

            if y < self.cells.len() && x < self.cells[y].len() {
                self.cells[y][x]
            } else {
                INVALID
            }
        } else {
            INVALID
        }
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn width(&self) -> usize {
        self.cells[0].len()
    }
}

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
    let grid = Grid::parse(input);

    (0..grid.height())
        .map(|y| {
            (0..grid.width())
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
                    .into_iter()
                    .filter(|dir| {
                        (0..3)
                            .fold(vec![coord], |mut acc, _| {
                                acc.push(dir(*acc.last().expect("not empty")));
                                acc
                            })
                            .into_iter()
                            .map(|coord| grid.get(coord))
                            .collect::<String>()
                            == "XMAS"
                    })
                    .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn p2(input: &str) -> String {
    let grid = Grid::parse(input);

    (1..grid.height() - 1)
        .map(|y| {
            (1..grid.width() - 1)
                .filter(|x| {
                    let coord = Coord(*x as i64, y as i64);

                    if grid.get(coord) == 'A' {
                        let ring = [
                            coord.upleft(),
                            coord.upright(),
                            coord.downright(),
                            coord.downleft(),
                        ]
                        .into_iter()
                        .map(|coord| grid.get(coord))
                        .collect::<String>();
                        ["MMSS", "SMMS", "SSMM", "MSSM"]
                            .into_iter()
                            .any(|candidate| ring == candidate)
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
