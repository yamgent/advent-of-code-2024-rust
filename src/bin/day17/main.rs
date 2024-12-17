const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/17/input.txt");

fn p1(input: &str) -> String {
    let (mut reg, program) = input.trim().lines().into_iter().enumerate().fold(
        (vec![], vec![]),
        |(mut reg, program), (line_number, line)| {
            match line_number {
                0 | 1 | 2 => {
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
    );

    let mut ptr = 0;
    let mut out = vec![];

    while ptr < program.len() {
        let opcode = program[ptr];

        let operand = program[ptr + 1];
        let is_literal_operand = matches!(operand, 0..=3);
        let operand_value = if is_literal_operand {
            operand
        } else if matches!(operand, 4..=6) {
            reg[(operand - 4) as usize]
        } else if operand == 7 {
            panic!("Operand 7 is reserved");
        } else {
            panic!("Illegal operand {}", operand);
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

    out.into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join(",")
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
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1,5,0,3,7,3,0,3,1");
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
