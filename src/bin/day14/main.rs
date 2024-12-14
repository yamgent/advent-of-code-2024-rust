use ahash::HashSet;

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/14/input.txt");

struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

trait AdvanceSimulation {
    fn advance(self, bathroom_size: (i64, i64), time: i64) -> Self;
}

fn parse_input(input: &str) -> Vec<Robot> {
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
                    .fold((0, 0), |acc, (idx, value)| {
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
        .collect()
}

impl AdvanceSimulation for Vec<Robot> {
    fn advance(self, bathroom_size: (i64, i64), time: i64) -> Self {
        self.into_iter()
            .map(|robot| Robot {
                pos: (
                    (robot.pos.0 + (robot.vel.0 * time)).rem_euclid(bathroom_size.0),
                    (robot.pos.1 + (robot.vel.1 * time)).rem_euclid(bathroom_size.1),
                ),
                vel: robot.vel,
            })
            .collect()
    }
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

    parse_input(input)
        .advance(bathroom_size, 100)
        .into_iter()
        .map(|robot| robot.pos)
        .flat_map(find_quadrant)
        .fold([0, 0, 0, 0], |mut acc, quadrant| {
            acc[quadrant] += 1;
            acc
        })
        .into_iter()
        .product::<usize>()
        .to_string()
}

const ACTUAL_BATHROOM_SIZE: (i64, i64) = (101, 103);

fn p1(input: &str) -> String {
    solve_p1(input, ACTUAL_BATHROOM_SIZE)
}

fn print_view(view: &HashSet<(i64, i64)>) {
    (0..ACTUAL_BATHROOM_SIZE.1).for_each(|y| {
        (0..ACTUAL_BATHROOM_SIZE.0)
            .for_each(|x| print!("{}", if view.contains(&(x, y)) { "*" } else { "." }));
        println!();
    });
}

fn p2(input: &str) -> String {
    let mut current = parse_input(input);

    for iteration in 1..10_000 {
        current = current.advance(ACTUAL_BATHROOM_SIZE, 1);

        let view: HashSet<(i64, i64)> = current.iter().map(|robot| robot.pos).collect();

        if view
            .iter()
            .find(|pos| (1..=10).all(|i| view.contains(&(pos.0, pos.1 + i))))
            .is_some()
        {
            println!("Iteration {}", iteration);
            print_view(&view);
            return iteration.to_string();
        }
    }

    panic!("cannot find christmas tree.");
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
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "8280");
    }
}
