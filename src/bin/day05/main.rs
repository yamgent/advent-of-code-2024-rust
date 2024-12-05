use std::collections::{HashMap, HashSet};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/05/input.txt");

#[derive(Clone)]
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
        self.children
            .entry(from)
            .or_default()
            .insert(to);
    }
}

fn p1(input: &str) -> String {
    let (rules, updates) = input
        .trim()
        .split_once("\n\n")
        .expect("input to have two sections as described in problem");

    let rules = rules
        .trim()
        .lines()
        .map(|line| line.split_once("|").expect("input to be of format a|b"))
        .map(|(parent, child)| {
            (
                parent.parse::<usize>().expect("positive integer"),
                child.parse::<usize>().expect("positive integer"),
            )
        })
        .fold(Graph::new(), |mut graph, (parent, child)| {
            graph.connect(parent, child);
            graph
        });

    updates
        .trim()
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|page| page.parse::<usize>().expect("positive integer"))
                .collect::<Vec<_>>()
        })
        .filter(|update| {
            let mut seen = HashSet::new();

            !update.iter().any(|page| {
                if let Some(children) = rules.children.get(page) {
                    if children.iter().any(|child_page| seen.contains(child_page)) {
                        return true;
                    }
                }
                seen.insert(page);
                false
            })
        })
        .map(|update| {
            let mid = update.len() / 2;
            *update.get(mid).unwrap()
        })
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
