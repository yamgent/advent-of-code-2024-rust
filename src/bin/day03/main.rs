use regex::Regex;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/03/input.txt");

fn p1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("valid regex");

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| {
            a.parse::<i64>().expect("a number") * b.parse::<i64>().expect("a number")
        })
        .sum::<i64>()
        .to_string()
}

fn p2(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("valid regex");

    re.captures_iter(input)
        .fold((0i64, true), |(mut acc, enabled), captures| {
            let matched = &captures[0];

            if matched == "do()" {
                (acc, true)
            } else if matched == "don't()" {
                (acc, false)
            } else {
                if enabled {
                    let a = &captures[1];
                    let b = &captures[2];
                    acc +=
                        a.parse::<i64>().expect("a number") * b.parse::<i64>().expect("a number");
                }
                (acc, enabled)
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
