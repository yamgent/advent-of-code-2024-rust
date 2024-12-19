use ahash::{HashMap, HashMapExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/19/input.txt");

struct Input<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}

fn parse_input(input: &str) -> Input {
    let (towels, patterns) = input
        .trim()
        .split_once("\n\n")
        .expect("input has two sections");

    let towels = towels.split(",").map(|s| s.trim()).collect::<Vec<_>>();
    let patterns = patterns.lines().map(|s| s.trim()).collect::<Vec<_>>();

    Input { towels, patterns }
}

fn p1(input: &str) -> String {
    let input = parse_input(input);

    input
        .patterns
        .into_iter()
        .filter(|pattern| {
            fn check(pattern: &str, towels: &[&str], current_idx: usize) -> bool {
                if current_idx == pattern.len() {
                    true
                } else {
                    towels.iter().any(|towel| {
                        if pattern.len() - current_idx >= towel.len()
                            && pattern[current_idx..(current_idx + towel.len())] == **towel
                        {
                            check(pattern, towels, current_idx + towel.len())
                        } else {
                            false
                        }
                    })
                }
            }

            check(pattern, &input.towels, 0)
        })
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    let input = parse_input(input);
    let mut dp = HashMap::new();

    input
        .patterns
        .into_iter()
        .map(|pattern| {
            fn count<'a>(
                dp: &mut HashMap<&'a str, usize>,
                pattern: &'a str,
                towels: &[&str],
                current_idx: usize,
            ) -> usize {
                if current_idx == pattern.len() {
                    1
                } else {
                    let substring = &pattern[current_idx..];
                    if let Some(count) = dp.get(&substring) {
                        *count
                    } else {
                        let count = towels
                            .iter()
                            .map(|towel| {
                                if pattern.len() - current_idx >= towel.len()
                                    && pattern[current_idx..(current_idx + towel.len())] == **towel
                                {
                                    count(dp, pattern, towels, current_idx + towel.len())
                                } else {
                                    0
                                }
                            })
                            .sum::<usize>();

                        dp.insert(substring, count);
                        count
                    }
                }
            }

            count(&mut dp, pattern, &input.towels, 0)
        })
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

    const SAMPLE_INPUT: &str = r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "6");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "340");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "16");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "717561822679428");
    }
}
