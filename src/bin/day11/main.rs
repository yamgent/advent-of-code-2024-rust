use ahash::{HashMap, HashMapExt};

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
    fn process(stone: u64, blink: usize, dp: &mut HashMap<(u64, usize), usize>) -> usize {
        if dp.contains_key(&(stone, blink)) {
            *dp.get(&(stone, blink)).unwrap()
        } else {
            let final_value = if blink == 0 {
                1
            } else if stone == 0 {
                process(1, blink - 1, dp)
            } else {
                let digit_count = total_digits(stone);

                if digit_count % 2 == 0 {
                    let upper = stone / 10u64.pow(digit_count as u32 / 2);
                    let lower = stone % (10u64.pow(digit_count as u32 / 2));
                    process(upper, blink - 1, dp) + process(lower, blink - 1, dp)
                } else {
                    process(stone * 2024, blink - 1, dp)
                }
            };

            dp.insert((stone, blink), final_value);
            final_value
        }
    }

    let mut dp = HashMap::new();

    input
        .trim()
        .split(" ")
        .map(|val| val.parse().expect("a number"))
        .map(|val| process(val, blink, &mut dp))
        .sum::<usize>()
        .to_string()
}

fn p1(input: &str) -> String {
    solve(input, 25)
}

fn p2(input: &str) -> String {
    solve(input, 75)
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
        assert_eq!(solve("0 1 10 99 999", 1), "7");
        assert_eq!(solve("125 17", 6), "22");
        assert_eq!(p1("125 17"), "55312");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "197157");
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "234430066982597");
    }
}
