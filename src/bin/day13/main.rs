use std::cell::LazyCell;

use regex::Regex;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/13/input.txt");

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

const MACHINE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .expect("valid regex")
});

// a_1 * x + b_1 * y = c_1
// a_2 * x + b_2 * y = c_2
fn solve_sim_eq(a_1: i64, b_1: i64, c_1: i64, a_2: i64, b_2: i64, c_2: i64) -> Option<(i64, i64)> {
    let det = a_1 * b_2 - a_2 * b_1;
    if det == 0 {
        None
    } else {
        let top = c_1 * b_2 + c_2 * -b_1;
        let bot = c_1 * -a_2 + c_2 * a_1;

        if top % det != 0 || bot % det != 0 {
            None
        } else {
            Some((top / det, bot / det))
        }
    }
}

fn p1(input: &str) -> String {
    input
        .trim()
        .split("\n\n")
        .map(|test_case| {
            MACHINE_REGEX
                .captures(test_case)
                .expect("a valid written test case")
                .extract::<6>()
                .1
                .into_iter()
                .map(|val| val.parse::<i64>().expect("a number"))
                .collect::<Vec<_>>()
        })
        .map(|values| Machine {
            a: (values[0], values[1]),
            b: (values[2], values[3]),
            prize: (values[4], values[5]),
        })
        .flat_map(|machine| {
            solve_sim_eq(
                machine.a.0,
                machine.b.0,
                machine.prize.0,
                machine.a.1,
                machine.b.1,
                machine.prize.1,
            )
        })
        .map(|(a, b)| a * 3 + b)
        .sum::<i64>()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .split("\n\n")
        .map(|test_case| {
            MACHINE_REGEX
                .captures(test_case)
                .expect("a valid written test case")
                .extract::<6>()
                .1
                .into_iter()
                .map(|val| val.parse::<i64>().expect("a number"))
                .collect::<Vec<_>>()
        })
        .map(|values| Machine {
            a: (values[0], values[1]),
            b: (values[2], values[3]),
            prize: (values[4] + 10000000000000, values[5] + 10000000000000),
        })
        .flat_map(|machine| {
            solve_sim_eq(
                machine.a.0,
                machine.b.0,
                machine.prize.0,
                machine.a.1,
                machine.b.1,
                machine.prize.1,
            )
        })
        .map(|(a, b)| a * 3 + b)
        .sum::<i64>()
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
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "480");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "37686");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "77204516023437");
    }
}
