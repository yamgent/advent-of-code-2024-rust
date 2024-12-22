use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/22/input.txt");

fn next_secret(mut secret: u64) -> u64 {
    fn mix_prune(secret: u64, number: u64) -> u64 {
        (secret ^ number) % 16777216
    }

    secret = mix_prune(secret, secret * 64);
    secret = mix_prune(secret, secret / 32);
    secret = mix_prune(secret, secret * 2048);
    secret
}

fn p1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().expect("a number"))
        .map(|number| (0..2000).fold(number, |acc, _| next_secret(acc)))
        .sum::<u64>()
        .to_string()
}

fn p2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().expect("a number"))
        .map(|number| {
            (0..2000).fold(vec![number], |mut acc, _| {
                acc.push(next_secret(*acc.last().unwrap()));
                acc
            })
        })
        .map(|numbers| {
            numbers
                .into_iter()
                .map(|number| number % 10)
                .collect::<Vec<_>>()
        })
        .map(|price| {
            let mut bananas = HashMap::new();

            let mut idx = 1;
            let mut sliding_window = VecDeque::new();

            while idx < price.len() {
                if sliding_window.len() == 4 {
                    let tuple = (
                        sliding_window[0],
                        sliding_window[1],
                        sliding_window[2],
                        sliding_window[3],
                    );
                    if !bananas.contains_key(&tuple) {
                        bananas.insert(tuple, price[idx - 1]);
                    }
                    sliding_window.pop_front();
                }
                sliding_window.push_back(price[idx] as i64 - price[idx - 1] as i64);
                idx += 1;
            }

            bananas
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<(i64, i64, i64, i64), u64>, monkey_bananas| {
                monkey_bananas.into_iter().for_each(|(key, value)| {
                    *acc.entry(key).or_default() += value;
                });
                acc
            },
        )
        .into_values()
        .max()
        .expect("input should have answer")
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
1
10
100
2024
";

    #[test]
    fn test_p1_sample() {
        let sequence_test = r"
123
15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254
"
        .trim()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

        sequence_test
            .iter()
            .zip(sequence_test.iter().skip(1))
            .for_each(|(start, end)| {
                assert_eq!(next_secret(*start), *end, "{} -> {}", start, end);
            });

        assert_eq!(p1(SAMPLE_INPUT), "37327623");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "15006633487");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2(r"
1
2
3
2024
"),
            "23"
        );
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "1710");
    }
}
