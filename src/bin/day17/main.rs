const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/17/input.txt");

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    input.trim().lines().enumerate().fold(
        (vec![], vec![]),
        |(mut reg, program), (line_number, line)| {
            match line_number {
                0..=2 => {
                    reg.push(
                        line.split(":")
                            .nth(1)
                            .expect(" xxxx")
                            .trim()
                            .parse::<i64>()
                            .expect("a number"),
                    );
                    (reg, program)
                }
                3 => {
                    // ignore
                    (reg, program)
                }
                4 => (
                    reg,
                    line.split(":")
                        .nth(1)
                        .expect(" x,x,x,x")
                        .trim()
                        .split(",")
                        .map(|x| x.parse::<i64>().expect("a number"))
                        .collect(),
                ),
                _ => {
                    panic!(
                        "Not expecting more than 5 lines for the input, but found line {}, with line {}.",
                        line_number, line
                    );
                }
            }
        },
    )
}

fn execute_program(mut reg: Vec<i64>, program: &[i64]) -> Vec<i64> {
    let mut ptr = 0;
    let mut out = vec![];

    while ptr < program.len() {
        let opcode = program[ptr];

        let operand = program[ptr + 1];
        let operand_value = match operand {
            0..=3 => operand,
            4..=6 => reg[(operand - 4) as usize],
            7 => panic!("Operand 7 is reserved"),
            _ => panic!("Illegal operand {}", operand),
        };

        let mut advance_ptr = true;

        match opcode {
            0 => {
                reg[0] /= 2_i64.pow(operand_value as u32);
            }
            1 => {
                reg[1] ^= operand;
            }
            2 => {
                reg[1] = operand_value % 8;
            }
            3 => {
                if reg[0] != 0 {
                    advance_ptr = false;
                    ptr = operand as usize;
                }
            }
            4 => {
                reg[1] ^= reg[2];
            }
            5 => {
                out.push(operand_value % 8);
            }
            6 => {
                reg[1] = reg[0] / 2_i64.pow(operand_value as u32);
            }
            7 => {
                reg[2] = reg[0] / 2_i64.pow(operand_value as u32);
            }
            _ => panic!("Invalid opcode {}", opcode),
        }

        if advance_ptr {
            ptr += 2;
        }
    }

    out
}

fn p1(input: &str) -> String {
    let (reg, program) = parse_input(input);
    execute_program(reg, &program)
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn p2(input: &str) -> String {
    /*
    {
        let (_, program) = parse_input(ACTUAL_INPUT);
        // https://www.reddit.com/r/adventofcode/comments/1hg69ql/2024_day_17_part_2_can_someone_please_provide_a/
        println!("{:?}", execute_program(vec![0, 0, 0], &program));
        println!("{:?}", execute_program(vec![1, 0, 0], &program));
        println!("{:?}", execute_program(vec![2, 0, 0], &program));
        println!("{:?}", execute_program(vec![3, 0, 0], &program));
        println!("{:?}", execute_program(vec![4, 0, 0], &program));
        println!("{:?}", execute_program(vec![5, 0, 0], &program));
        println!("{:?}", execute_program(vec![6, 0, 0], &program));
        println!("{:?}", execute_program(vec![7, 0, 0], &program));
        println!("{:?}", execute_program(vec![8, 0, 0], &program));
        println!();
        println!("{:?}", execute_program(vec![9, 0, 0], &program));
        println!("{:?}", execute_program(vec![10, 0, 0], &program));
        println!("{:?}", execute_program(vec![11, 0, 0], &program));
        println!("{:?}", execute_program(vec![12, 0, 0], &program));
        println!("{:?}", execute_program(vec![13, 0, 0], &program));
        println!("{:?}", execute_program(vec![14, 0, 0], &program));
        println!("{:?}", execute_program(vec![15, 0, 0], &program));
        println!();
        println!("{:?}", execute_program(vec![16, 0, 0], &program));
        println!("{:?}", execute_program(vec![17, 0, 0], &program));
        println!("{:?}", execute_program(vec![18, 0, 0], &program));
        println!("{:?}", execute_program(vec![19, 0, 0], &program));
        println!("{:?}", execute_program(vec![20, 0, 0], &program));
        println!("{:?}", execute_program(vec![21, 0, 0], &program));
        println!("{:?}", execute_program(vec![22, 0, 0], &program));
        println!("{:?}", execute_program(vec![23, 0, 0], &program));
        println!();
        println!("{:?}", execute_program(vec![64, 0, 0], &program));
        println!("{:?}", execute_program(vec![512, 0, 0], &program));
        println!("{:?}", execute_program(vec![4096, 0, 0], &program));
        println!("{:?}", execute_program(vec![32768, 0, 0], &program));
        println!("{:?}", execute_program(vec![262144, 0, 0], &program));
        println!("{:?}", execute_program(vec![2097152, 0, 0], &program));
        println!("{:?}", execute_program(vec![16777216, 0, 0], &program));
        println!("{:?}", execute_program(vec![16777217, 0, 0], &program));
        println!("{:?}", execute_program(vec![16777218, 0, 0], &program));
        println!();

        let len_pow = program.len() - 1;
        let start = 8_i64.pow(len_pow as u32);
        let mut rest = (0..len_pow).map(|v| v as i64).collect::<Vec<_>>();

        fn compute(start: i64, rest: Vec<i64>) -> i64 {
            start + rest.iter().rev().fold(0, |acc, val| acc * 8 + val)
        }

        for i in 0..rest.len() {
            rest[i] = (0..8)
                .find(|v| {
                    let final_rest = rest
                        .iter()
                        .enumerate()
                        .map(|(j, x)| if j == i { *v } else { *x })
                        .collect::<Vec<_>>();
                    execute_program(vec![compute(start, final_rest), 0, 0], &program)[i]
                        == program[i]
                })
                .expect("input is valid");
            println!("{:?}", rest);
        }

        compute(start, rest).to_string();
    }
    */

    let (_, program) = parse_input(input);

    (0..(program.len() / 2))
        .fold(vec![], |mut acc, i| {
            let ptr = i * 2;
            let opcode = program[ptr];

            let operand = program[ptr + 1];
            let operand_value = match operand {
                0..=3 => operand.to_string(),
                4 => "A".to_string(),
                5 => "B".to_string(),
                6 => "C".to_string(),
                7 => panic!("Operand 7 is reserved"),
                _ => panic!("Illegal operand {}", operand),
            };

            acc.push(match opcode {
                0 => {
                    format!("{}: A = A >> {}", ptr, operand_value)
                }
                1 => {
                    format!("{}: B ^= {}", ptr, operand)
                }
                2 => {
                    format!("{}: B = {} % 8", ptr, operand_value)
                }
                3 => {
                    format!("{}: A != 0 -> goto {}", ptr, operand)
                }
                4 => {
                    format!("{}: B ^= C", ptr)
                }
                5 => {
                    format!("{}: out({} % 8)", ptr, operand_value)
                }
                6 => {
                    format!("{}: B = A >> {}", ptr, operand_value)
                }
                7 => {
                    format!("{}: C = A >> {}", ptr, operand_value)
                }
                _ => panic!("Invalid opcode {}", opcode),
            });

            acc
        })
        .join("\n")
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const P2_EXAMPLE: &str = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(P1_EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1,5,0,3,7,3,0,3,1");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(
            p2(P1_EXAMPLE),
            r"
0: A = A >> 1
2: out(A % 8)
4: A != 0 -> goto 0
"
            .trim()
        );
        assert_eq!(
            p2(P2_EXAMPLE),
            r"
0: A = A >> 3
2: out(A % 8)
4: A != 0 -> goto 0
"
            .trim()
        );

        assert_eq!(
            p2(r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,0,4,1,4,2,1,2,5,3,1,4,0,5,3,5,6,6,1,7,1
"),
            r"
0: A = A >> 1
2: A = A >> A
4: B ^= 4
6: B = 1 % 8
8: B = B % 8
10: A != 0 -> goto 1
12: B ^= C
14: out(3 % 8)
16: out(C % 8)
18: B = A >> 1
20: C = A >> 1
"
            .trim()
        );

        let (_, program) = parse_input(P2_EXAMPLE);
        let answer = 117440;
        assert!(execute_program(vec![answer, 0, 0], &program) == program);
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        let (_, program) = parse_input(ACTUAL_INPUT);
        let answer = 0;
        assert!(execute_program(vec![answer, 0, 0], &program) == program);
    }
}
