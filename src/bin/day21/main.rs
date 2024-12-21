use std::{collections::VecDeque, sync::LazyLock};

use ahash::{HashMap, HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/21/input.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DPad {
    Up,
    Down,
    Left,
    Right,
    A,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum NPad {
    Number(usize),
    A,
}

trait CoordAdvance
where
    Self: Sized,
{
    fn up(&self) -> Option<Self>;
    fn down(&self, bounds: (usize, usize)) -> Option<Self>;
    fn left(&self) -> Option<Self>;
    fn right(&self, bounds: (usize, usize)) -> Option<Self>;
}

impl CoordAdvance for (usize, usize) {
    fn up(&self) -> Option<Self> {
        self.1.checked_sub(1).map(|y| (self.0, y))
    }

    fn down(&self, bounds: (usize, usize)) -> Option<Self> {
        if self.1 + 1 < bounds.1 {
            Some((self.0, self.1 + 1))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Self> {
        self.0.checked_sub(1).map(|x| (x, self.1))
    }

    fn right(&self, bounds: (usize, usize)) -> Option<Self> {
        if self.0 + 1 < bounds.0 {
            Some((self.0 + 1, self.1))
        } else {
            None
        }
    }
}

impl NPad {
    fn handle_dpad_press(&self, dpad: DPad) -> Option<Self> {
        static POS_TO_NPAD: LazyLock<HashMap<(usize, usize), NPad>> = LazyLock::new(|| {
            [
                ((0, 0), NPad::Number(7)),
                ((1, 0), NPad::Number(8)),
                ((2, 0), NPad::Number(9)),
                ((0, 1), NPad::Number(4)),
                ((1, 1), NPad::Number(5)),
                ((2, 1), NPad::Number(6)),
                ((0, 2), NPad::Number(1)),
                ((1, 2), NPad::Number(2)),
                ((2, 2), NPad::Number(3)),
                ((1, 3), NPad::Number(0)),
                ((2, 3), NPad::A),
            ]
            .into_iter()
            .collect()
        });
        static NPAD_TO_POS: LazyLock<HashMap<NPad, (usize, usize)>> =
            LazyLock::new(|| POS_TO_NPAD.iter().map(|(k, v)| (*v, *k)).collect());
        static BOUNDS: (usize, usize) = (3, 4);

        let current_pos = NPAD_TO_POS.get(self).unwrap();
        let new_pos = match dpad {
            DPad::Up => current_pos.up(),
            DPad::Down => current_pos.down(BOUNDS),
            DPad::Left => current_pos.left(),
            DPad::Right => current_pos.right(BOUNDS),
            DPad::A => return Some(*self),
        };

        new_pos.and_then(|pos| POS_TO_NPAD.get(&pos).copied())
    }
}

impl DPad {
    fn handle_dpad_press(&self, dpad: DPad) -> Option<Self> {
        static POS_TO_DPAD: LazyLock<HashMap<(usize, usize), DPad>> = LazyLock::new(|| {
            [
                ((1, 0), DPad::Up),
                ((2, 0), DPad::A),
                ((0, 1), DPad::Left),
                ((1, 1), DPad::Down),
                ((2, 1), DPad::Right),
            ]
            .into_iter()
            .collect()
        });
        static DPAD_TO_POS: LazyLock<HashMap<DPad, (usize, usize)>> =
            LazyLock::new(|| POS_TO_DPAD.iter().map(|(k, v)| (*v, *k)).collect());
        static BOUNDS: (usize, usize) = (3, 2);

        let current_pos = DPAD_TO_POS.get(self).unwrap();
        let new_pos = match dpad {
            DPad::Up => current_pos.up(),
            DPad::Down => current_pos.down(BOUNDS),
            DPad::Left => current_pos.left(),
            DPad::Right => current_pos.right(BOUNDS),
            DPad::A => return Some(*self),
        };

        new_pos.and_then(|pos| POS_TO_DPAD.get(&pos).copied())
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct StateNode {
    npad: NPad,
    dpads: Vec<DPad>,
    successes: usize,
}

impl StateNode {
    fn start_state(total_dpads: usize) -> Self {
        Self {
            npad: NPad::A,
            dpads: std::iter::repeat(DPad::A).take(total_dpads).collect(),
            successes: 0,
        }
    }

    fn handle_dpad_press(&self, dpad: DPad, sequence: &str) -> Option<Self> {
        if self.successes >= sequence.len() {
            None
        } else if dpad != DPad::A {
            self.dpads[0].handle_dpad_press(dpad).map(|dpad| {
                let mut new_dpads = self.dpads.clone();
                new_dpads[0] = dpad;
                Self {
                    dpads: new_dpads,
                    ..*self
                }
            })
        } else {
            match self
                .dpads
                .iter()
                .enumerate()
                .find(|(_, dpad)| **dpad != DPad::A)
            {
                Some((dpad_idx, non_a_dpad)) => {
                    if dpad_idx == self.dpads.len() - 1 {
                        self.npad.handle_dpad_press(*non_a_dpad).map(|npad| Self {
                            dpads: self.dpads.clone(),
                            npad,
                            ..*self
                        })
                    } else {
                        self.dpads[dpad_idx + 1].handle_dpad_press(*non_a_dpad).map(
                            |affected_dpad| {
                                let mut new_dpads = self.dpads.clone();
                                new_dpads[dpad_idx + 1] = affected_dpad;
                                Self {
                                    dpads: new_dpads,
                                    ..*self
                                }
                            },
                        )
                    }
                }
                None => {
                    let sequence_char = sequence.chars().nth(self.successes).unwrap();

                    match (sequence_char, self.npad) {
                        ('A', NPad::A) => Some(Self {
                            successes: self.successes + 1,
                            dpads: self.dpads.clone(),
                            ..*self
                        }),
                        ('0'..='9', NPad::Number(npad_number)) => {
                            let sequence_number = sequence_char.to_digit(10).unwrap() as usize;
                            if npad_number == sequence_number {
                                Some(Self {
                                    successes: self.successes + 1,
                                    dpads: self.dpads.clone(),
                                    ..*self
                                })
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
            }
        }
    }

    fn is_end_state(&self, sequence: &str) -> bool {
        self.successes == sequence.len()
    }
}

fn find_shortest(line: &str, total_dpads: usize) -> usize {
    let mut visited: HashSet<StateNode> = HashSet::new();

    let mut to_process = [(0, StateNode::start_state(total_dpads))]
        .into_iter()
        .collect::<VecDeque<_>>();

    while let Some(next) = to_process.pop_front() {
        if next.1.is_end_state(line) {
            return next.0;
        }

        if visited.contains(&next.1) {
            continue;
        }

        visited.insert(next.1.clone());

        [DPad::Up, DPad::Down, DPad::Left, DPad::Right, DPad::A]
            .into_iter()
            .flat_map(|human_dpad| next.1.handle_dpad_press(human_dpad, line))
            .for_each(|new_node| {
                to_process.push_back((next.0 + 1, new_node));
            });
    }

    panic!("input should have an answer")
}

fn get_numeric(line: &str) -> usize {
    assert!(line.len() == 4);
    line[0..3].parse::<usize>().expect("input should be xxxA")
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| find_shortest(line, 2) * get_numeric(line))
        .sum::<usize>()
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
029A
980A
179A
456A
379A
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "126384");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "248684");
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
