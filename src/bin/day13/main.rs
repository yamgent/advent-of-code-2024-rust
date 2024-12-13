const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/13/input.txt");

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

// a_1 * x + b_1 * y = c_1
// a_2 * x + b_2 * y = c_2
fn solve_sim_eq(a_1: i64, b_1: i64, c_1: i64, a_2: i64, b_2: i64, c_2: i64) -> Option<(i64, i64)> {
    let det = a_1 * b_2 - a_2 * b_1;
    let top = c_1 * b_2 + c_2 * -b_1;
    let bot = c_1 * -a_2 + c_2 * a_1;

    if det == 0 || top % det != 0 || bot % det != 0 {
        None
    } else {
        Some((top / det, bot / det))
    }
}

fn solve(input: &str, prize_correction: i64) -> i64 {
    input
        .trim()
        .split("\n\n")
        .map(|test_case| {
            let lines = test_case.lines().collect::<Vec<_>>();
            let mut results = Vec::with_capacity(6);

            lines[0]
                .split(":")
                .nth(1)
                .expect(" X+xxx, Y+xxx")
                .split(",")
                .map(|part| {
                    part.trim()
                        .split("+")
                        .nth(1)
                        .expect("a number")
                        .parse::<i64>()
                        .expect("a number")
                })
                .for_each(|val| {
                    results.push(val);
                });
            lines[1]
                .split(":")
                .nth(1)
                .expect(" X+xxx, Y+xxx")
                .split(",")
                .map(|part| {
                    part.trim()
                        .split("+")
                        .nth(1)
                        .expect("a number")
                        .parse::<i64>()
                        .expect("a number")
                })
                .for_each(|val| {
                    results.push(val);
                });
            lines[2]
                .split(":")
                .nth(1)
                .expect(" X=xxx, Y=xxx")
                .split(",")
                .map(|part| {
                    part.trim()
                        .split("=")
                        .nth(1)
                        .expect("a number")
                        .parse::<i64>()
                        .expect("a number")
                })
                .for_each(|val| {
                    results.push(val);
                });

            results
        })
        .map(|values| Machine {
            a: (values[0], values[1]),
            b: (values[2], values[3]),
            prize: (values[4] + prize_correction, values[5] + prize_correction),
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
}

fn p1(input: &str) -> String {
    solve(input, 0).to_string()
}

fn p2(input: &str) -> String {
    solve(input, 10000000000000).to_string()
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
