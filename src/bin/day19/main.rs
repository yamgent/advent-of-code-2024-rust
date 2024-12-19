const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/19/input.txt");

fn p1(input: &str) -> String {
    let (towels, patterns) = input
        .trim()
        .split_once("\n\n")
        .expect("input has two sections");

    let towels = towels.split(",").map(|s| s.trim()).collect::<Vec<_>>();
    let patterns = patterns.lines().map(|s| s.trim()).collect::<Vec<_>>();

    patterns
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

            check(pattern, &towels, 0)
        })
        .count()
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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
