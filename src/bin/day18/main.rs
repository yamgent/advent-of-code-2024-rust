use std::collections::VecDeque;

use ahash::{HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/18/input.txt");

fn go_left(coord: (usize, usize)) -> Option<(usize, usize)> {
    coord.0.checked_sub(1).map(|x| (x, coord.1))
}

fn go_up(coord: (usize, usize)) -> Option<(usize, usize)> {
    coord.1.checked_sub(1).map(|y| (coord.0, y))
}

fn go_right(coord: (usize, usize), bounds: (usize, usize)) -> Option<(usize, usize)> {
    if coord.0 + 1 >= bounds.0 {
        None
    } else {
        Some((coord.0 + 1, coord.1))
    }
}

fn go_down(coord: (usize, usize), bounds: (usize, usize)) -> Option<(usize, usize)> {
    if coord.1 + 1 >= bounds.1 {
        None
    } else {
        Some((coord.0, coord.1 + 1))
    }
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|x| x.trim().split_once(",").expect("x,x"))
        .map(|(x, y)| {
            (
                x.parse::<usize>().expect("a number"),
                (y.parse::<usize>().expect("a number")),
            )
        })
        .collect()
}

fn get_shortest_path(points: &HashSet<(usize, usize)>, bounds: (usize, usize)) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut to_process = VecDeque::new();
    to_process.push_back((0, (0, 0)));

    while let Some(current) = to_process.pop_front() {
        if current.1 == (bounds.0 - 1, bounds.1 - 1) {
            return Some(current.0);
        }

        [
            go_left(current.1),
            go_right(current.1, bounds),
            go_up(current.1),
            go_down(current.1, bounds),
        ]
        .into_iter()
        .for_each(|pos| {
            if let Some(pos) = pos {
                if !points.contains(&pos) && !visited.contains(&pos) {
                    visited.insert(pos);
                    to_process.push_back((current.0 + 1, pos));
                }
            }
        });
    }

    None
}

fn solve_p1(input: &str, bounds: (usize, usize), bytes_fallen: usize) -> String {
    let points = parse_input(input)
        .into_iter()
        .take(bytes_fallen)
        .collect::<HashSet<_>>();

    get_shortest_path(&points, bounds)
        .expect("input should always have an answer")
        .to_string()
}

fn p1(input: &str) -> String {
    solve_p1(input, (71, 71), 1024)
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
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve_p1(SAMPLE_INPUT, (7, 7), 12), "22");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "304");
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
