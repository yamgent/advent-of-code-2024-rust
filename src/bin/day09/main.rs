const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/09/input.txt");

fn p1(input: &str) -> String {
    const NOT_ALLOCATED: usize = usize::MAX;

    let values = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("a digit"))
        .collect::<Vec<_>>();

    let capacity = values.iter().sum::<u32>() as usize;

    let mut filesystem = values
        .into_iter()
        .enumerate()
        .fold(
            (Vec::with_capacity(capacity), true),
            |(mut acc, is_file), (idx, val)| {
                if is_file {
                    let id = idx / 2;
                    (0..val).for_each(|_| acc.push(id));
                } else {
                    (0..val).for_each(|_| acc.push(NOT_ALLOCATED));
                }
                (acc, !is_file)
            },
        )
        .0;

    let mut left = filesystem
        .iter()
        .enumerate()
        .find(|(_, val)| **val == NOT_ALLOCATED)
        .expect("at least one empty space")
        .0;

    let mut right = filesystem
        .iter()
        .enumerate()
        .rev()
        .find(|(_, val)| **val != NOT_ALLOCATED)
        .expect("at least one allocated file")
        .0;

    while left < right {
        filesystem[left] = filesystem[right];
        filesystem[right] = NOT_ALLOCATED;
        while left < filesystem.len() && filesystem[left] != NOT_ALLOCATED {
            left += 1;
        }
        while right > 0 && filesystem[right] == NOT_ALLOCATED {
            right -= 1;
        }
    }

    filesystem
        .into_iter()
        .enumerate()
        .filter(|(_, id)| *id != NOT_ALLOCATED)
        .map(|(pos, id)| pos * id)
        .sum::<usize>()
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

    const SAMPLE_INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "1928");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "6385338159127");
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
