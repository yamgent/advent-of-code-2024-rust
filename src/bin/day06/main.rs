use ahash::AHashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/06/input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: Vec2,
    dir: Dir,
}

impl Guard {
    fn advance(self, map: &Map) -> Option<Self> {
        let in_front_is_out_of_bounds = match self.dir {
            Dir::Up => self.pos.1 == 0,
            Dir::Down => self.pos.1 == map.bounds.1 - 1,
            Dir::Left => self.pos.0 == 0,
            Dir::Right => self.pos.0 == map.bounds.0 - 1,
        };

        if in_front_is_out_of_bounds {
            None
        } else {
            let front_pos = match self.dir {
                Dir::Up => Vec2(self.pos.0, self.pos.1 - 1),
                Dir::Down => Vec2(self.pos.0, self.pos.1 + 1),
                Dir::Left => Vec2(self.pos.0 - 1, self.pos.1),
                Dir::Right => Vec2(self.pos.0 + 1, self.pos.1),
            };

            if map.obstacles.contains(&front_pos) {
                Some(Self {
                    pos: self.pos,
                    dir: match self.dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    },
                })
            } else {
                Some(Self {
                    pos: front_pos,
                    dir: self.dir,
                })
            }
        }
    }
}

#[derive(Clone)]
struct Map {
    bounds: Vec2,
    guard_start: Guard,
    obstacles: AHashSet<Vec2>,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        let grid = input.trim().lines().collect::<Vec<_>>();

        let bounds = Vec2(grid[0].len(), grid.len());
        let guard_start = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.chars().enumerate().find_map(|(x, ch)| {
                    if ch == '^' {
                        Some(Guard {
                            pos: Vec2(x, y),
                            dir: Dir::Up,
                        })
                    } else {
                        None
                    }
                })
            })
            .expect("guard present in input");
        let obstacles = grid
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '#')
                    .map(|(x, _)| Vec2(x, y))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            bounds,
            guard_start,
            obstacles,
        }
    }
}

fn get_guard_p1_path(map: &Map) -> AHashSet<Vec2> {
    let mut visited = AHashSet::new();
    let mut updated_guard_pos = Some(map.guard_start);

    while let Some(guard) = updated_guard_pos {
        visited.insert(guard.pos);
        updated_guard_pos = guard.advance(map);
    }

    visited
}

fn p1(input: &str) -> String {
    let map = Map::parse_input(input);
    get_guard_p1_path(&map).len().to_string()
}

fn guard_stuck_in_loop(map: &Map) -> bool {
    let mut visited = AHashSet::new();
    let mut updated_guard_pos = Some(map.guard_start);

    while let Some(guard) = updated_guard_pos {
        if visited.contains(&guard) {
            return true;
        }

        updated_guard_pos = guard.advance(map);

        // we don't have to insert EVERY position the guard visited,
        // only the positions where the guard has to make a turn
        //
        // this optimization alone cut the processing time by HALF
        if let Some(updated_guard_pos) = updated_guard_pos {
            if updated_guard_pos.pos == guard.pos {
                visited.insert(guard);
            }
        }
    }

    false
}

fn p2(input: &str) -> String {
    let map = Map::parse_input(input);
    get_guard_p1_path(&map)
        .into_iter()
        .filter(|pos| *pos != map.guard_start.pos)
        .filter(|pos| {
            let mut new_map = map.clone();
            new_map.obstacles.insert(*pos);
            guard_stuck_in_loop(&new_map)
        })
        .count()
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "41");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "5239");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "6");
    }

    #[test]
    #[ignore = "inefficient algorithm, took 4s on local"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1753");
    }
}
