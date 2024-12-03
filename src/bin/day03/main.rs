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

    const SAMPLE_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "161");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "166630675");
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
