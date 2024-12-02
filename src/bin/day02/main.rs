const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/02/input.txt");

fn is_safe_increasing(values: &[i32]) -> bool {
    values
        .iter()
        .zip(values.iter().skip(1))
        .all(|(a, b)| b - a >= 1 && b - a <= 3)
}

fn is_safe_decreasing(values: &[i32]) -> bool {
    values
        .iter()
        .zip(values.iter().skip(1))
        .all(|(a, b)| a - b >= 1 && a - b <= 3)
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().expect("a number"))
                .collect::<Vec<_>>()
        })
        .filter(|line| is_safe_increasing(line) || is_safe_decreasing(line))
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().expect("a number"))
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            is_safe_increasing(line)
                || is_safe_decreasing(line)
                || (0..line.len()).any(|skip_idx| {
                    let new_values = line
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| *idx != skip_idx)
                        .map(|(_, val)| *val)
                        .collect::<Vec<_>>();
                    is_safe_increasing(&new_values) || is_safe_decreasing(&new_values)
                })
        })
        .count()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "2");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "421");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "4");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "476");
    }
}
