const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/07/input.txt");

fn total_digits(value: u64) -> usize {
    if value == 0 {
        1
    } else {
        (value.ilog10() + 1) as usize
    }
}

fn equation_solvable(coefficients: &[u64], expected: u64, can_concat: bool) -> bool {
    fn compute(
        coefficients: &[u64],
        expected: u64,
        can_concat: bool,
        expected_digits: usize,
        index: usize,
        acc: u64,
    ) -> bool {
        if index >= coefficients.len() {
            acc == expected
        } else if acc > expected {
            // only compare acc "greater than" expected, not "equal", because if
            // the remaining numbers are 0 and/or 1, then can still + or *
            // to get the answer
            false
        } else if compute(
            coefficients,
            expected,
            can_concat,
            expected_digits,
            index + 1,
            acc + coefficients[index],
        ) || compute(
            coefficients,
            expected,
            can_concat,
            expected_digits,
            index + 1,
            acc * coefficients[index],
        ) {
            true
        } else if !can_concat {
            false
        } else {
            let left_length = total_digits(acc);
            let right_length = total_digits(coefficients[index]);

            if left_length + right_length > expected_digits {
                false
            } else {
                compute(
                    coefficients,
                    expected,
                    can_concat,
                    expected_digits,
                    index + 1,
                    acc * 10u64.pow(right_length as u32) + coefficients[index],
                )
            }
        }
    }
    if coefficients.is_empty() {
        expected == 0
    } else {
        compute(
            coefficients,
            expected,
            can_concat,
            total_digits(expected),
            0,
            0,
        )
    }
}

fn solve(input: &str, can_concat: bool) -> String {
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
        .filter(|(test_value, coefficients)| {
            equation_solvable(coefficients, *test_value, can_concat)
        })
        .map(|(test_value, _)| test_value)
        .sum::<u64>()
        .to_string()
}

fn p1(input: &str) -> String {
    solve(input, false)
}

fn p2(input: &str) -> String {
    solve(input, true)
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
