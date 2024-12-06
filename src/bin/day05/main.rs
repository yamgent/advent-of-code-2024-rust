use std::collections::{HashMap, HashSet};

// input commentary: actual input's rulebook have cyclic paths, so using topo sort on the rulebook
// would not work for both part 1 and 2. However, the updates themselves are specially crafted to
// not cause a cyclic loop, so it might work on the updates?? never tried though
const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/05/input.txt");

struct Graph {
    children: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    fn connect(&mut self, from: usize, to: usize) {
        self.children.entry(from).or_default().insert(to);
    }
}

fn parse_input(input: &str) -> (Graph, Vec<Vec<usize>>) {
    let (rules, updates) = input
        .trim()
        .split_once("\n\n")
        .expect("input to have two sections as described in problem");

    (
        rules
            .trim()
            .lines()
            .map(|line| line.split_once("|").expect("input to be of format a|b"))
            .map(|(parent, child)| {
                (
                    parent.parse().expect("positive integer"),
                    child.parse().expect("positive integer"),
                )
            })
            .fold(Graph::new(), |mut graph, (parent, child)| {
                graph.connect(parent, child);
                graph
            }),
        updates
            .trim()
            .lines()
            .map(|update| {
                update
                    .split(",")
                    .map(|page| page.parse().expect("positive integer"))
                    .collect()
            })
            .collect(),
    )
}

fn is_valid_ordering(rules: &Graph, update: &[usize]) -> bool {
    let mut seen = HashSet::new();

    let found_violated_order = update.iter().any(|page| {
        seen.insert(page);
        rules
            .children
            .get(page)
            .map(|children| children.iter().any(|child_page| seen.contains(child_page)))
            .unwrap_or(false)
    });

    !found_violated_order
}

fn get_middle_page(update: &[usize]) -> usize {
    *update.get(update.len() / 2).unwrap()
}

fn p1(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    updates
        .into_iter()
        .filter(|update| is_valid_ordering(&rules, update))
        .map(|update| get_middle_page(&update))
        .sum::<usize>()
        .to_string()
}

fn fix_page_ordering(rules: &Graph, update: &[usize]) -> Vec<usize> {
    let mut result = update.to_vec();

    let empty_hashset = HashSet::new();

    for i in 0..result.len() {
        let children = rules.children.get(&result[i]).unwrap_or(&empty_hashset);

        if let Some(smallest_violate_idx) =
            (0..i)
                .rev()
                .fold(None, |smallest_violate_idx_so_far, inspect_idx| {
                    if children.contains(&result[inspect_idx]) {
                        Some(inspect_idx)
                    } else {
                        smallest_violate_idx_so_far
                    }
                })
        {
            (smallest_violate_idx..i).rev().for_each(|idx| {
                result.swap(idx, idx + 1);
            });
        }
    }

    assert!(is_valid_ordering(rules, &result));
    result
}

fn p2(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    updates
        .into_iter()
        .filter(|update| !is_valid_ordering(&rules, update))
        .map(|update| fix_page_ordering(&rules, &update))
        .map(|update| get_middle_page(&update))
        .sum::<usize>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "143");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "4924");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "123");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "6085");
    }
}
