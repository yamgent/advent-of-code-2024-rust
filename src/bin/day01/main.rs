use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/01/input.txt");

fn p1(input: &str) -> String {
    let (mut left_list, mut right_list) =
        input
            .trim()
            .lines()
            .fold((vec![], vec![]), |mut acc, line| {
                let values = line.split_whitespace().collect::<Vec<_>>();

                if values.len() != 2 {
                    panic!("Expect two values per line, but {line} is not");
                }

                acc.0.push(values[0].parse::<i32>().expect("not a number"));
                acc.1.push(values[1].parse::<i32>().expect("not a number"));

                acc
            });

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .into_iter()
        .zip(right_list.into_iter())
        .map(|(left, right)| left.max(right) - left.min(right))
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let (left_list, right_map) =
        input
            .trim()
            .lines()
            .fold((vec![], HashMap::new()), |mut acc, line| {
                let values = line.split_whitespace().collect::<Vec<_>>();

                if values.len() != 2 {
                    panic!("Expect two values per line, but {line} is not");
                }

                acc.0.push(values[0].parse::<i32>().expect("not a number"));

                let right_number = values[1].parse::<i32>().expect("not a number");
                *(acc.1.entry(right_number).or_insert(0)) += 1;

                acc
            });

    left_list
        .into_iter()
        .map(|left_number| left_number * (right_map.get(&left_number).copied().unwrap_or_default()))
        .sum::<i32>()
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "11");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1882714");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "31");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "19437052");
    }
}
