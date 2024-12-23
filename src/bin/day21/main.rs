use std::{collections::VecDeque, sync::LazyLock};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

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
    // solution from: https://www.reddit.com/r/adventofcode/comments/1hjx0x4/2024_day_21_quick_tutorial_to_solve_part_2_in/
    // ^ without this, was difficult to sovle

    fn bfs(
        map: &HashMap<(i32, i32), char>,
        rev_map: &HashMap<char, (i32, i32)>,
        start: char,
        end: char,
    ) -> Vec<Vec<char>> {
        let end_pos = rev_map.get(&end).expect("valid node");

        let mut result = vec![];

        let mut visited = HashSet::new();
        let mut to_process = vec![(*rev_map.get(&start).expect("valid node"), vec![])];
        let mut found_end = false;

        while !found_end {
            let mut next_level = vec![];

            while let Some(next_node) = to_process.pop() {
                if next_node.0 == *end_pos {
                    found_end = true;
                    result.push(next_node.1);
                } else {
                    visited.insert(next_node.0);

                    [
                        ((next_node.0 .0 - 1, next_node.0 .1), '<'),
                        ((next_node.0 .0 + 1, next_node.0 .1), '>'),
                        ((next_node.0 .0, next_node.0 .1 - 1), '^'),
                        ((next_node.0 .0, next_node.0 .1 + 1), 'v'),
                    ]
                    .into_iter()
                    .filter(|neighbour| !visited.contains(&neighbour.0))
                    .filter(|neighbour| map.get(&neighbour.0).is_some())
                    .for_each(|neighbour| {
                        let mut stack = next_node.1.clone();
                        stack.push(neighbour.1);
                        next_level.push((neighbour.0, stack));
                    });
                }
            }

            to_process = next_level;
        }

        result
    }

    let numpad_map = {
        let pos_to_npad = [
            ((0, 0), '7'),
            ((1, 0), '8'),
            ((2, 0), '9'),
            ((0, 1), '4'),
            ((1, 1), '5'),
            ((2, 1), '6'),
            ((0, 2), '1'),
            ((1, 2), '2'),
            ((2, 2), '3'),
            ((1, 3), '0'),
            ((2, 3), 'A'),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        let npad_to_pos = pos_to_npad
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect::<HashMap<_, _>>();

        let mut map = HashMap::new();
        (0..=9).for_each(|num| {
            let num = char::from_digit(num, 10).unwrap();

            map.insert(('A', num), bfs(&pos_to_npad, &npad_to_pos, 'A', num));
            map.insert((num, 'A'), bfs(&pos_to_npad, &npad_to_pos, num, 'A'));
        });
        (0..=9).for_each(|start| {
            let start = char::from_digit(start, 10).unwrap();

            (0..=9).for_each(|end| {
                let end = char::from_digit(end, 10).unwrap();

                map.insert((start, end), bfs(&pos_to_npad, &npad_to_pos, start, end));
                map.insert((end, start), bfs(&pos_to_npad, &npad_to_pos, end, start));
            });
        });
        map
    };

    assert_eq!(
        numpad_map
            .get(&('7', '0'))
            .unwrap()
            .iter()
            .cloned()
            .collect::<HashSet<_>>(),
        [">vvv", "v>vv", "vv>v"]
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
    );

    let dirpad_map = {
        let pos_to_dpad = [
            ((1, 0), '^'),
            ((2, 0), 'A'),
            ((0, 1), '<'),
            ((1, 1), 'v'),
            ((2, 1), '>'),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        let dpad_to_pos = pos_to_dpad
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect::<HashMap<_, _>>();

        let mut map = HashMap::new();

        ['^', 'v', '<', '>', 'A'].into_iter().for_each(|start| {
            ['^', 'v', '<', '>', 'A'].into_iter().for_each(|end| {
                map.insert((start, end), bfs(&pos_to_dpad, &dpad_to_pos, start, end));
                map.insert((end, start), bfs(&pos_to_dpad, &dpad_to_pos, end, start));
            });
        });
        map
    };

    fn build_seq(
        keys: &[char],
        index: usize,
        prev_key: char,
        curr_path: Vec<char>,
        result: &mut Vec<Vec<char>>,
        maps: &HashMap<(char, char), Vec<Vec<char>>>,
    ) {
        if index >= keys.len() {
            result.push(curr_path);
        } else {
            maps.get(&(prev_key, keys[index]))
                .unwrap_or_else(|| panic!("valid map and keys {} {}", prev_key, keys[index]))
                .iter()
                .for_each(|path| {
                    let mut next_path = curr_path.clone();
                    next_path.extend(path);
                    next_path.push('A');

                    build_seq(keys, index + 1, keys[index], next_path, result, maps);
                });
        }
    }

    {
        let mut test_result = vec![];
        build_seq(&['<', 'A'], 0, 'A', vec![], &mut test_result, &dirpad_map);

        assert_eq!(
            test_result.into_iter().collect::<HashSet<_>>(),
            ["<v<A>>^A", "<v<A>^>A", "v<<A>>^A", "v<<A>^>A",]
                .into_iter()
                .map(|line| line.chars().collect())
                .collect()
        );
    }

    fn shortest_seq(
        keys: &[char],
        depth: usize,
        cache: &mut HashMap<(Vec<char>, usize), usize>,
        dirpad_map: &HashMap<(char, char), Vec<Vec<char>>>,
    ) -> usize {
        if depth == 0 {
            keys.len()
        } else if let Some(value) = cache.get(&(keys.to_vec(), depth)) {
            *value
        } else {
            let total = keys
                .iter()
                .fold(vec![vec![]], |mut acc, key| {
                    acc.last_mut().unwrap().push(*key);
                    if *key == 'A' {
                        acc.push(vec![]);
                    }
                    acc
                })
                .into_iter()
                .fold(0, |acc, subkey| {
                    let mut seqs = vec![];
                    build_seq(&subkey, 0, 'A', vec![], &mut seqs, dirpad_map);
                    acc + seqs
                        .iter()
                        .map(|seq| shortest_seq(seq, depth - 1, cache, dirpad_map))
                        .min()
                        .expect("one valid seq")
                });

            cache.insert((keys.to_vec(), depth), total);
            total
        }
    }

    let mut shortest_seq_cache = HashMap::new();
    input
        .trim()
        .lines()
        .fold(0, |acc, line| {
            let line = line.trim().chars().collect::<Vec<_>>();
            let mut line_result = vec![];
            build_seq(&line, 0, 'A', vec![], &mut line_result, &numpad_map);

            let min = line_result
                .iter()
                .map(|list| shortest_seq(list, 25, &mut shortest_seq_cache, &dirpad_map))
                .min()
                .expect("one valid seq");

            let num = line[0..(line.len() - 1)]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .expect("a number");

            acc + (min * num)
        })
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
        assert_eq!(p2(SAMPLE_INPUT), "154115708116294");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "307055584161760");
    }
}
