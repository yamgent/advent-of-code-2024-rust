const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/15/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn parse_input(ch: char) -> Self {
        match ch {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!(
                "Unrecognized character {} that should not be in the input.",
                ch
            ),
        }
    }

    fn advance(
        &self,
        current_position: (usize, usize),
        bounds: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Move::Up => {
                if current_position.1 == 0 {
                    None
                } else {
                    Some((current_position.0, current_position.1 - 1))
                }
            }
            Move::Down => {
                if current_position.1 + 1 >= bounds.1 {
                    None
                } else {
                    Some((current_position.0, current_position.1 + 1))
                }
            }
            Move::Left => {
                if current_position.0 == 0 {
                    None
                } else {
                    Some((current_position.0 - 1, current_position.1))
                }
            }
            Move::Right => {
                if current_position.0 + 1 >= bounds.0 {
                    None
                } else {
                    Some((current_position.0 + 1, current_position.1))
                }
            }
        }
    }
}

struct Input {
    grid: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let (grid, moves) = input
            .trim()
            .split_once("\n\n")
            .expect("test case should have two sections separated by a newline");

        Self {
            grid: grid
                .trim()
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect(),
            moves: moves
                .trim()
                .lines()
                .flat_map(|line| line.chars().map(Move::parse_input).collect::<Vec<_>>())
                .collect(),
        }
    }

    fn simulate_p1(mut self) -> usize {
        let mut robot_position = self
            .grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, ch)| if *ch == '@' { Some(x) } else { None })
                    .map(|x| (x, y))
            })
            .expect("input should have a robot");
        let bounds = (self.grid[0].len(), self.grid.len());

        self.moves.iter().for_each(|move_dir| {
            let next_pos = move_dir.advance(robot_position, bounds);

            if let Some(next_pos) = next_pos {
                let next_ch = self.grid[next_pos.1][next_pos.0];

                if next_ch == '.' {
                    self.grid[robot_position.1][robot_position.0] = '.';
                    self.grid[next_pos.1][next_pos.0] = '@';
                    robot_position = next_pos;
                } else if next_ch == 'O' {
                    let mut box_replace_pos_candidate = Some(next_pos);

                    while let Some(box_replace_pos) = box_replace_pos_candidate {
                        match self.grid[box_replace_pos.1][box_replace_pos.0] {
                            '.' => break,
                            '#' => {
                                box_replace_pos_candidate = None;
                                break;
                            }
                            'O' => {
                                box_replace_pos_candidate =
                                    move_dir.advance(box_replace_pos, bounds);
                            }
                            _ => {}
                        }
                    }

                    if let Some(box_replace_pos) = box_replace_pos_candidate {
                        self.grid[box_replace_pos.1][box_replace_pos.0] = 'O';
                        self.grid[robot_position.1][robot_position.0] = '.';
                        self.grid[next_pos.1][next_pos.0] = '@';
                        robot_position = next_pos;
                    }
                }
            }
        });

        self.grid
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, ch)| if ch == 'O' { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
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

    const SAMPLE_INPUT: &str = r"";

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1(r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"),
            "2028"
        );
        assert_eq!(
            p1(r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"),
            "10092"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1511865");
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
