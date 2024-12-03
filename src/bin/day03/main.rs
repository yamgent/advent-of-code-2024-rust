use regex::{Captures, Regex};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/03/input.txt");

fn p1(input: &str) -> String {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .expect("valid regex")
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .map(|(_, values)| {
            values
                .iter()
                .map(|v| v.parse::<i64>().expect("a number"))
                .product::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

enum Statement {
    Multiply(i64, i64),
    Do,
    Dont,
}

impl Statement {
    fn parse(captures: Captures) -> Self {
        let text = &captures[0];

        match text {
            "do()" => Statement::Do,
            "don't()" => Statement::Dont,
            _ => {
                let a = &captures[1].parse::<i64>().expect("a number");
                let b = &captures[2].parse::<i64>().expect("a number");
                Statement::Multiply(*a, *b)
            }
        }
    }
}

fn p2(input: &str) -> String {
    Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
        .expect("valid regex")
        .captures_iter(input)
        .map(Statement::parse)
        .fold((0i64, true), |(acc, enabled), statement| match statement {
            Statement::Do => (acc, true),
            Statement::Dont => (acc, false),
            Statement::Multiply(a, b) => {
                if enabled {
                    (acc + a * b, enabled)
                } else {
                    (acc, enabled)
                }
            }
        })
        .0
        .to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sample() {
        assert_eq!(
            p1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            "161"
        );
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "166630675");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            "48"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "93465710");
    }
}
