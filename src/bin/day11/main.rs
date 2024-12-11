const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/11/input.txt");

// TODO: Repeated code
fn total_digits(value: u64) -> usize {
    if value == 0 {
        1
    } else {
        (value.ilog10() + 1) as usize
    }
}

fn solve(input: &str, blink: usize) -> String {
    fn process(stone: u64, blink: usize) -> usize {
        if blink == 0 {
            1
        } else if stone == 0 {
            process(1, blink - 1)
        } else {
            let digit_count = total_digits(stone);

            if digit_count % 2 == 0 {
                let upper = stone / 10u64.pow(digit_count as u32 / 2);
                let lower = stone % (10u64.pow(digit_count as u32 / 2));
                process(upper, blink - 1) + process(lower, blink - 1)
            } else {
                process(stone * 2024, blink - 1)
            }
        }
    }

    input
        .trim()
        .split(" ")
        .map(|val| val.parse().expect("a number"))
        .map(|val| process(val, blink))
        .sum::<usize>()
        .to_string()
}

fn p1(input: &str) -> String {
    solve(input, 25)
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

    const SAMPLE_INPUT: &str = r"";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve("0 1 10 99 999", 1), "7");
        assert_eq!(solve("125 17", 6), "22");
        assert_eq!(p1("125 17"), "55312");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "197157");
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
