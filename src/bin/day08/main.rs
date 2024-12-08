use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/08/input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord(i64, i64);

impl Coord {
    fn sub(&self, other: Coord) -> Coord {
        Coord(self.0 - other.0, self.1 - other.1)
    }

    fn mul(&self, scalar: i64) -> Coord {
        Coord(self.0 * scalar, self.1 * scalar)
    }

    fn in_bounds(&self, bounds: &Coord) -> bool {
        self.0 >= 0 && self.0 < bounds.0 && self.1 >= 0 && self.1 < bounds.1
    }
}

struct Map {
    antennas: HashMap<char, Vec<Coord>>,
    bounds: Coord,
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
                            acc.entry(ch).or_insert(vec![]).push(Coord(x, y));
                        }
                    });

                    acc
                },
            ),
            bounds: Coord(
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
        .iter()
        .map(|(_, pos)| {
            pos.iter()
                .flat_map(|a| pos.iter().map(|b| (a, b)).collect::<Vec<_>>())
                .filter(|(x, y)| x != y)
                .map(|(x, y)| x.mul(2).sub(*y))
                .filter(|pos| pos.in_bounds(&map.bounds))
                .collect::<HashSet<_>>()
        })
        .fold(HashSet::new(), |mut acc, positions| {
            acc.extend(positions);
            acc
        })
        .len()
        .to_string()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
