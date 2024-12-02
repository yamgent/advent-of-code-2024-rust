const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/02/input.txt");

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().expect("a number"))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_safe_increasing(values: &[i32]) -> bool {
    values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| b - a)
        .all(|diff| diff >= 1 && diff <= 3)
}

fn is_safe_decreasing(values: &[i32]) -> bool {
    values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| a - b)
        .all(|diff| diff >= 1 && diff <= 3)
}

fn p1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .filter(|values| is_safe_increasing(values) || is_safe_decreasing(values))
        .count()
        .to_string()
}

fn p2(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .filter(|values| {
            is_safe_increasing(values)
                || is_safe_decreasing(values)
                || (0..values.len()).any(|skip_idx| {
                    let new_values = values
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
