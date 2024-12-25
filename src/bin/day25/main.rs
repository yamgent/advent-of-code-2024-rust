const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/25/input.txt");

const BOUNDS_WIDTH: usize = 5;
const BOUNDS_HEIGHT: usize = 7;

#[derive(Debug)]
struct Key {
    heights: [u64; BOUNDS_WIDTH],
}

#[derive(Debug)]
struct Lock {
    heights: [u64; BOUNDS_WIDTH],
}

fn parse_input(input: &str) -> (Vec<Key>, Vec<Lock>) {
    input
        .trim()
        .split("\n\n")
        .fold((vec![], vec![]), |(mut keys, mut locks), section| {
            let section = section.trim().lines().collect::<Vec<_>>();

            let heights = (0..section[0].len())
                .map(|col| {
                    section
                        .iter()
                        .filter(|line| {
                            line.chars()
                                .nth(col)
                                .expect("all lines to have same length")
                                == '#'
                        })
                        .count() as u64
                })
                .collect::<Vec<_>>();

            if section[0] == "#####" {
                locks.push(Lock {
                    //heights: heights.into(),
                    heights: heights.as_slice().try_into().expect("length BOUNDS_WIDTH"),
                });
            } else {
                keys.push(Key {
                    heights: heights.as_slice().try_into().expect("length BOUNDS_WIDTH"),
                });
            }

            (keys, locks)
        })
}

fn p1(input: &str) -> String {
    let (keys, locks) = parse_input(input);

    keys.into_iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    key.heights
                        .iter()
                        .zip(lock.heights)
                        .map(|(l, k)| l + k)
                        .all(|h| h <= BOUNDS_HEIGHT as u64)
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "3");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "3307");
    }
}
