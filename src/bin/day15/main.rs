use std::collections::VecDeque;

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

    fn expand_p2(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        grid.into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|ch| match ch {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => panic!("Unrecognized character {} in grid", ch),
                    })
                    .collect()
            })
            .collect()
    }

    fn gps_p2(grid: Vec<Vec<char>>) -> usize {
        grid.into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, ch)| if ch == '[' { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }

    fn move_p2(mut grid: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
        let mut robot_position = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, ch)| if *ch == '@' { Some(x) } else { None })
                    .map(|x| (x, y))
            })
            .expect("input grid should have a robot");
        let bounds = (grid[0].len(), grid.len());

        moves.into_iter().for_each(|current_move| {
            if let Some(final_robot_pos) = current_move.advance(robot_position, bounds) {
                if grid[final_robot_pos.1][final_robot_pos.0] == '.' {
                    grid[final_robot_pos.1][final_robot_pos.0] = '@';
                    grid[robot_position.1][robot_position.0] = '.';
                    robot_position = final_robot_pos;
                } else if matches!(grid[final_robot_pos.1][final_robot_pos.0], '[' | ']') {
                    match current_move {
                        Move::Up | Move::Down => {
                            let final_robot_fat_pos =
                                if grid[final_robot_pos.1][final_robot_pos.0] == '[' {
                                    (final_robot_pos.0, final_robot_pos.1)
                                } else {
                                    (final_robot_pos.0 - 1, final_robot_pos.1)
                                };

                            let mut affected_boxes = vec![];

                            let mut to_process =
                                [final_robot_fat_pos].into_iter().collect::<VecDeque<_>>();

                            let mut can_move = true;

                            while can_move && !to_process.is_empty() {
                                let current_process = to_process.pop_front().unwrap();
                                let next_y =
                                    current_move.advance(current_process, bounds).unwrap().1;

                                affected_boxes.push(current_process);

                                if grid[next_y][current_process.0] == '.'
                                    && grid[next_y][current_process.0 + 1] == '.'
                                {
                                    continue;
                                }
                                if grid[next_y][current_process.0] == '#'
                                    || grid[next_y][current_process.0 + 1] == '#'
                                {
                                    can_move = false;
                                    break;
                                }
                                if grid[next_y][current_process.0] == '[' {
                                    to_process.push_back((current_process.0, next_y));
                                }
                                if grid[next_y][current_process.0] == ']' {
                                    to_process.push_back((current_process.0 - 1, next_y));
                                }
                                if grid[next_y][current_process.0 + 1] == '[' {
                                    to_process.push_back((current_process.0 + 1, next_y));
                                }
                            }

                            if can_move {
                                affected_boxes.iter().for_each(|box_pos| {
                                    grid[box_pos.1][box_pos.0] = '.';
                                    grid[box_pos.1][box_pos.0 + 1] = '.';
                                });
                                affected_boxes.into_iter().for_each(|box_pos| {
                                    let next_y = current_move.advance(box_pos, bounds).unwrap().1;
                                    grid[next_y][box_pos.0] = '[';
                                    grid[next_y][box_pos.0 + 1] = ']';
                                });

                                grid[final_robot_fat_pos.1][final_robot_fat_pos.0] = '.';
                                grid[final_robot_fat_pos.1][final_robot_fat_pos.0 + 1] = '.';

                                grid[final_robot_pos.1][final_robot_pos.0] = '@';
                                grid[robot_position.1][robot_position.0] = '.';
                                robot_position = final_robot_pos;
                            }
                        }
                        Move::Left | Move::Right => {
                            let mut final_box_position = Some(final_robot_pos);

                            while let Some(pos) = final_box_position {
                                if grid[pos.1][pos.0] == '#' {
                                    final_box_position = None;
                                    break;
                                }
                                if grid[pos.1][pos.0] == '.' {
                                    break;
                                }

                                final_box_position = current_move.advance(pos, bounds);
                                if let Some(pos) = final_box_position {
                                    final_box_position = current_move.advance(pos, bounds);
                                }
                            }

                            if let Some(pos) = final_box_position {
                                let (start, end) = if pos.0 < final_robot_pos.0 {
                                    (pos.0, final_robot_pos.0)
                                } else {
                                    ((final_robot_pos.0 + 1), pos.0 + 1)
                                };

                                (start..end).for_each(|x| {
                                    grid[final_robot_pos.1][x] =
                                        if (x - start) % 2 == 0 { '[' } else { ']' }
                                });
                                grid[final_robot_pos.1][final_robot_pos.0] = '@';
                                grid[robot_position.1][robot_position.0] = '.';
                                robot_position = final_robot_pos;
                            }
                        }
                    }
                }
            }
        });

        grid
    }

    fn simulate_p2(mut self) -> usize {
        self.grid = Input::expand_p2(self.grid);
        self.grid = Input::move_p2(self.grid, self.moves);
        Input::gps_p2(self.grid)
    }
}

fn p1(input: &str) -> String {
    Input::parse_input(input).simulate_p1().to_string()
}

fn p2(input: &str) -> String {
    Input::parse_input(input).simulate_p2().to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALLER_EXAMPLE: &str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const LARGER_EXAMPLE: &str = r"
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
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SMALLER_EXAMPLE), "2028");
        assert_eq!(p1(LARGER_EXAMPLE), "10092");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1511865");
    }

    fn get_grid_string(grid: &Vec<Vec<char>>) -> String {
        grid.iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_p2_expand() {
        let input = Input::parse_input(LARGER_EXAMPLE);
        assert_eq!(
            get_grid_string(&Input::expand_p2(input.grid)),
            r"
####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################
"
            .trim()
        );
    }

    #[test]
    fn test_p2_gps() {
        let input = Input::parse_input(
            r"
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################

^
",
        );
        assert_eq!(Input::gps_p2(input.grid), 9021);
    }

    #[test]
    fn test_p2_move() {
        fn move_test_case(idx: usize, input: &str, expected: &str) {
            let input = Input::parse_input(input);
            assert_eq!(
                get_grid_string(&Input::move_p2(input.grid, input.moves)),
                expected.trim(),
                "idx: {}",
                idx
            );
        }

        [
            // 0
            (
                r"
#########
##..@..##
#########

<
",
                r"
#########
##.@...##
#########
",
            ),
            // 1
            (
                r"
#########
##..@..##
#########

>
",
                r"
#########
##...@.##
#########
",
            ),
            // 2
            (
                r"
#########
##..@..##
##.....##
#########

v
",
                r"
#########
##.....##
##..@..##
#########
",
            ),
            // 3
            (
                r"
#########
##.....##
##..@..##
#########

^
",
                r"
#########
##..@..##
##.....##
#########
",
            ),
            // 4
            (
                r"
#########
##.....##
##..@#.##
#########

>
",
                r"
#########
##.....##
##..@#.##
#########
",
            ),
            // 5
            (
                r"
#############
##.........##
##..@[][][]##
##.........##
#############

>
",
                r"
#############
##.........##
##..@[][][]##
##.........##
#############
",
            ),
            // 6
            (
                r"
#############
##.........##
##..@[][]..##
##.........##
#############

>
",
                r"
#############
##.........##
##...@[][].##
##.........##
#############
",
            ),
            // 7
            (
                r"
#############
##.........##
##..@[].#..##
##.........##
#############

>
",
                r"
#############
##.........##
##...@[]#..##
##.........##
#############
",
            ),
            // 8
            (
                r"
#############
##.........##
##[][][]@..##
##.........##
#############

<
",
                r"
#############
##.........##
##[][][]@..##
##.........##
#############
",
            ),
            // 9
            (
                r"
#############
##.........##
##..[][]@..##
##.........##
#############

<
",
                r"
#############
##.........##
##.[][]@...##
##.........##
#############
",
            ),
            // 10
            (
                r"
#############
##.........##
##..#.[]@..##
##.........##
#############

<
",
                r"
#############
##.........##
##..#[]@...##
##.........##
#############
",
            ),
            // 11
            (
                r"
#############
##...#.....##
##...[]....##
##....@....##
##.........##
#############

^
",
                r"
#############
##...#.....##
##...[]....##
##....@....##
##.........##
#############
",
            ),
            // 12
            (
                r"
#############
##..#..#...##
##...[]....##
##....@....##
##.........##
#############

^
",
                r"
#############
##..#[]#...##
##....@....##
##.........##
##.........##
#############
",
            ),
            // 13
            (
                r"
#############
##..#..#...##
##...[]....##
##...[]....##
##...@.....##
#############

^
",
                r"
#############
##..#[]#...##
##...[]....##
##...@.....##
##.........##
#############
",
            ),
            // 14
            (
                r"
#############
##.#....#..##
##..[][]...##
##...[]....##
##....@....##
#############

^
",
                r"
#############
##.#[][]#..##
##...[]....##
##....@....##
##.........##
#############
",
            ),
            // 15
            (
                r"
#############
##.##...#..##
##..[][]...##
##...[]....##
##....@....##
#############

^
",
                r"
#############
##.##...#..##
##..[][]...##
##...[]....##
##....@....##
#############
",
            ),
            // 16
            (
                r"
#############
##.#...##..##
##..[][]...##
##...[]....##
##....@....##
#############

^
",
                r"
#############
##.#...##..##
##..[][]...##
##...[]....##
##....@....##
#############
",
            ),
            // 17
            (
                r"
#############
###..##..####
##.[]..[]..##
##..[][]...##
##...[]....##
##....@....##
#############

^
",
                r"
#############
###[]##[]####
##..[][]...##
##...[]....##
##....@....##
##.........##
#############
",
            ),
            // 18
            (
                r"
#############
###..##.#####
##.[]..[]..##
##..[][]...##
##...[]....##
##....@....##
#############

^
",
                r"
#############
###..##.#####
##.[]..[]..##
##..[][]...##
##...[]....##
##....@....##
#############
",
            ),
            // 19
            (
                r"
##############
#............#
#..[].[].[]..#
#...[]..[]...#
#....[][]....#
#.....[].....#
#......@.....#
##############

^
",
                r"
##############
#..[]....[]..#
#...[][][]...#
#....[][]....#
#.....[].....#
#......@.....#
#............#
##############
",
            ),
            // 20: we are too lazy to repeat all "up" cases for the "down" cases, we assume that
            //     the logic is the "same". So we only have two "down" cases here.
            (
                r"
##############
#......@.....#
#...[][][]...#
#....[][]....#
#.....[].....#
#............#
##############

v
",
                r"
##############
#............#
#...[].@[]...#
#.....[].....#
#....[][]....#
#.....[].....#
##############
",
            ),
            // 21: second "down" case.
            (
                r"
##############
#......@.....#
#...[][][]...#
#....[][]....#
#.....[].....#
#......#.....#
##############

v
",
                r"
##############
#......@.....#
#...[][][]...#
#....[][]....#
#.....[].....#
#......#.....#
##############
",
            ),
        ]
        .iter()
        .enumerate()
        .for_each(|(idx, (input, expected))| {
            move_test_case(idx, input, expected);
        });
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(LARGER_EXAMPLE), "9021");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1519991");
    }
}
