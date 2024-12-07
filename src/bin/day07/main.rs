const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/07/input.txt");

fn solvable(coefficients: &[u64], expected: u64) -> bool {
    fn compute(coefficients: &[u64], expected: u64, index: usize, acc: u64) -> bool {
        if index >= coefficients.len() {
            acc == expected
        } else {
            compute(coefficients, expected, index + 1, acc + coefficients[index])
                || compute(coefficients, expected, index + 1, acc * coefficients[index])
        }
    }
    if coefficients.len() == 0 {
        expected == 0
    } else {
        compute(coefficients, expected, 0, 0)
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let (test_value, coefficients) = line
                .trim()
                .split_once(": ")
                .expect("line to be format xxx: xxx xxx");
            (
                test_value.parse::<u64>().expect("a number"),
                coefficients
                    .split(" ")
                    .map(|val| val.parse::<u64>().expect("a number"))
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(test_value, coefficients)| solvable(coefficients, *test_value))
        .map(|(test_value, _)| test_value)
        .sum::<u64>()
        .to_string()
}

fn total_digits(value: u64) -> usize {
    value.to_string().len()
}

fn solvable_p2(coefficients: &[u64], expected: u64) -> bool {
    fn compute(
        coefficients: &[u64],
        expected: u64,
        expected_digits: usize,
        index: usize,
        acc: u64,
    ) -> bool {
        if index >= coefficients.len() {
            acc == expected
        } else if compute(
            coefficients,
            expected,
            expected_digits,
            index + 1,
            acc + coefficients[index],
        ) || compute(
            coefficients,
            expected,
            expected_digits,
            index + 1,
            acc * coefficients[index],
        ) {
            true
        } else {
            let left_length = total_digits(acc);
            let right_length = total_digits(coefficients[index]);

            if left_length + right_length > expected_digits {
                false
            } else {
                compute(
                    coefficients,
                    expected,
                    expected_digits,
                    index + 1,
                    acc * 10u64.pow(right_length as u32) + coefficients[index],
                )
            }
        }
    }
    if coefficients.len() == 0 {
        expected == 0
    } else {
        compute(coefficients, expected, total_digits(expected), 0, 0)
    }
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let (test_value, coefficients) = line
                .trim()
                .split_once(": ")
                .expect("line to be format xxx: xxx xxx");
            (
                test_value.parse::<u64>().expect("a number"),
                coefficients
                    .split(" ")
                    .map(|val| val.parse::<u64>().expect("a number"))
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(test_value, coefficients)| solvable_p2(coefficients, *test_value))
        .map(|(test_value, _)| test_value)
        .sum::<u64>()
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
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "3749");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "12940396350192");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "11387");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "106016735664498");
    }
}
