use std::collections::HashMap;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/01/input.txt");

fn read_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.trim().split_once("   ").unwrap_or_else(|| {
                panic!("Expect two values per line with three spaces in between, but {line} is not")
            });

            (
                left.parse().expect("a number"),
                right.parse().expect("a number"),
            )
        })
        .fold((vec![], vec![]), |mut acc, line| {
            acc.0.push(line.0);
            acc.1.push(line.1);
            acc
        })
}

fn p1(input: &str) -> String {
    let (mut left_list, mut right_list) = read_input(input);

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>()
        .to_string()
}

fn p2(input: &str) -> String {
    let (left_list, right_list) = read_input(input);
    let right_map = right_list
        .into_iter()
        .fold(HashMap::new(), |mut acc, number| {
            *(acc.entry(number).or_insert(0)) += 1;
            acc
        });

    left_list
        .into_iter()
        .map(|left_number| left_number * (*right_map.get(&left_number).unwrap_or(&0)))
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
