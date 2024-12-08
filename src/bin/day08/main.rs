use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use ahash::{HashMap, HashMapExt, HashSet};
use itertools::Itertools;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/08/input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2i(i64, i64);

impl Add for Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2i {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<i64> for Vec2i {
    type Output = Vec2i;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<i64> for Vec2i {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

fn in_bounds(coord: &Vec2i, bounds: &Vec2i) -> bool {
    coord.0 >= 0 && coord.0 < bounds.0 && coord.1 >= 0 && coord.1 < bounds.1
}

struct Map {
    antennas: HashMap<char, Vec<Vec2i>>,
    bounds: Vec2i,
}

impl Map {
    fn parse_input(input: &str) -> Self {
        Self {
            antennas: input.trim().lines().enumerate().fold(
                HashMap::new(),
                |mut acc, (y, line)| {
                    let y = y as i64;

                    line.trim().chars().enumerate().for_each(|(x, ch)| {
                        if ch != '.' {
                            let x = x as i64;
                            acc.entry(ch).or_default().push(Vec2i(x, y));
                        }
                    });

                    acc
                },
            ),
            bounds: Vec2i(
                input
                    .trim()
                    .lines()
                    .nth(0)
                    .expect("non-empty input")
                    .chars()
                    .count() as i64,
                input.trim().lines().count() as i64,
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
