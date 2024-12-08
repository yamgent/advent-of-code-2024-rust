use ahash::{HashMap, HashMapExt, HashSet};
use glam::IVec2;
use itertools::Itertools;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/08/input.txt");

fn in_bounds(coord: &IVec2, bounds: &IVec2) -> bool {
    coord.x >= 0 && coord.x < bounds.x && coord.y >= 0 && coord.y < bounds.y
}

struct Map {
    antennas: HashMap<char, Vec<IVec2>>,
    bounds: IVec2,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        Self {
            antennas: input.trim().lines().enumerate().fold(
                HashMap::new(),
                |mut acc, (y, line)| {
                    let y = y as i32;

                    line.trim().chars().enumerate().for_each(|(x, ch)| {
                        if ch != '.' {
                            let x = x as i32;
                            acc.entry(ch).or_default().push(IVec2::new(x, y));
                        }
                    });

                    acc
                },
            ),
            bounds: IVec2::new(
                input
                    .trim()
                    .lines()
                    .nth(0)
                    .expect("non-empty input")
                    .chars()
                    .count() as i32,
                input.trim().lines().count() as i32,
            ),
        }
    }
}

fn p1(input: &str) -> String {
    let map = Map::parse_input(input);

    map.antennas
        .values()
        .flat_map(|pos| {
            pos.iter()
                .permutations(2)
                .flat_map(|points| {
                    let x = *points[0];
                    let y = *points[1];
                    vec![x * 2 - y]
                })
                .filter(|pos| in_bounds(pos, &map.bounds))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

fn p2(input: &str) -> String {
    let map = Map::parse_input(input);

    map.antennas
        .values()
        .flat_map(|pos| {
            pos.iter()
                .permutations(2)
                .flat_map(|points| {
                    let x = *points[0];
                    let y = *points[1];

                    let mut result = vec![];
                    let dir = x - y;
                    let mut current = y;

                    while in_bounds(&current, &map.bounds) {
                        result.push(current);
                        current += dir;
                    }

                    result
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
        .len()
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "14");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "336");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "34");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1131");
    }
}
