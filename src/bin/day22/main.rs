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
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
