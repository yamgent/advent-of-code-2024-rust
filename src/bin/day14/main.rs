const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/14/input.txt");

struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn solve_p1(input: &str, bathroom_size: (i64, i64)) -> String {
    let find_quadrant = |pos: (i64, i64)| -> Option<usize> {
        let mid = (bathroom_size.0 / 2, bathroom_size.1 / 2);

        match pos.0.cmp(&mid.0) {
            std::cmp::Ordering::Less => match pos.1.cmp(&mid.1) {
                std::cmp::Ordering::Less => Some(0),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(2),
            },
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => match pos.1.cmp(&mid.1) {
                std::cmp::Ordering::Less => Some(1),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(3),
            },
        }
    };

    input
        .trim()
        .lines()
        .map(|line| {
            let (p, v) = line
                .trim()
                .split_once(" ")
                .expect("line delimited by a single space, left is p, right is v");

            fn parse_vec2i(input: &str) -> (i64, i64) {
                input
                    .split("=")
                    .nth(1)
                    .expect("x,x")
                    .split(",")
                    .map(|value| value.parse::<i64>().expect("a number"))
                    .enumerate()
                    .fold((0i64, 0i64), |acc, (idx, value)| {
                        if idx == 0 {
                            (value, acc.1)
                        } else if idx == 1 {
                            (acc.0, value)
                        } else {
                            panic!("Found extra numbers, but was not expecting more than 2");
                        }
                    })
            }

            Robot {
                pos: parse_vec2i(p),
                vel: parse_vec2i(v),
            }
        })
        .map(|robot| {
            (
                (robot.pos.0 + (robot.vel.0 * 100)).rem_euclid(bathroom_size.0),
                (robot.pos.1 + (robot.vel.1 * 100)).rem_euclid(bathroom_size.1),
            )
        })
        .flat_map(find_quadrant)
        .fold([0, 0, 0, 0], |mut acc, quadrant| {
            acc[quadrant] += 1;
            acc
        })
        .into_iter()
        .product::<usize>()
        .to_string()
}

fn p1(input: &str) -> String {
    solve_p1(input, (101, 103))
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
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve_p1(SAMPLE_INPUT, (11, 7)), "12");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "231019008");
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
