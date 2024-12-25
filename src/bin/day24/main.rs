use ahash::{HashMap, HashMapExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/24/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    inputs: [String; 2],
    output: String,
    operator: Operator,
}

struct Input {
    wires: HashMap<String, usize>,
    gates: HashMap<String, Vec<Gate>>,
}

impl Input {
    fn parse_input(input: &str) -> Self {
        let (wires, gates) = input
            .trim()
            .split_once("\n\n")
            .expect("input has two sections");

        let wires = wires
            .trim()
            .lines()
            .map(|line| line.split_once(": ").expect("xXX: X"))
            .map(|(wire, val)| (wire.to_string(), val.parse::<usize>().expect("0 or 1")))
            .collect::<HashMap<_, _>>();

        let gates = gates
            .trim()
            .lines()
            .map(|line| line.split(" ").collect::<Vec<_>>())
            .map(|line| Gate {
                inputs: [line[0].to_string(), line[2].to_string()],
                output: line[4].to_string(),
                operator: match line[1] {
                    "AND" => Operator::And,
                    "OR" => Operator::Or,
                    "XOR" => Operator::Xor,
                    _ => panic!("Unknown operator {}", line[1]),
                },
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<String, Vec<Gate>>, gate| {
                    acc.entry(gate.inputs[0].clone())
                        .or_default()
                        .push(gate.clone());
                    acc.entry(gate.inputs[1].clone())
                        .or_default()
                        .push(gate.clone());
                    acc
                },
            );

        Self { wires, gates }
    }
}

fn p1(input: &str) -> String {
    let input = Input::parse_input(input);
    let mut wires = input.wires;
    let gates = input.gates;

    let mut to_process = wires.keys().cloned().collect::<Vec<_>>();

    while let Some(current_wire) = to_process.pop() {
        if let Some(current_wire_gates) = gates.get(&current_wire) {
            current_wire_gates.iter().for_each(|gate| {
                let other_gate = if gate.inputs[0] == current_wire {
                    &gate.inputs[1]
                } else {
                    &gate.inputs[0]
                };

                if let Some(other_val) = wires.get(other_gate) {
                    let current_val = wires.get(&current_wire).expect("visited before");
                    wires.insert(
                        gate.output.clone(),
                        match gate.operator {
                            Operator::And => current_val & other_val,
                            Operator::Or => current_val | other_val,
                            Operator::Xor => current_val ^ other_val,
                        },
                    );
                    to_process.push(gate.output.clone());
                }
            });
        }
    }

    let mut current_z_wire_number = 0;
    let mut current_z_wire = format!("z0{}", current_z_wire_number);
    let mut acc = 0;

    while let Some(value) = wires.get(&current_z_wire) {
        acc += value * 2usize.pow(current_z_wire_number);
        current_z_wire_number += 1;
        current_z_wire = format!(
            "z{}{}",
            if current_z_wire_number > 9 { "" } else { "0" },
            current_z_wire_number
        );
    }

    acc.to_string()
}

fn get_z(number: usize) -> String {
    format!("z{}{}", if number > 9 { "" } else { "0" }, number)
}

fn get_y(number: usize) -> String {
    format!("y{}{}", if number > 9 { "" } else { "0" }, number)
}

fn get_x(number: usize) -> String {
    format!("x{}{}", if number > 9 { "" } else { "0" }, number)
}

// FOR P2:
// manual solve and eyeball
//
// equations:
// x_n ^ y_n = insum_n
// carry_{n-1} ^ insum_n = sum_n (zNN)
// x_n & y_n = subcarry_n
// carry_{n-1} & insum_n = supercarry_n
// subcarry_n | supercarry_n = carry_n
const P2_CURE: [(&str, &str); 4] = [
    ("z10", "kmb"),
    ("z15", "tvp"),
    ("z25", "dpg"),
    ("mmf", "vdk"),
];

fn cure_p2(mut input: Input) -> Input {
    P2_CURE.iter().for_each(|cure| {
        input.gates.values_mut().for_each(|gates| {
            gates
                .iter_mut()
                .filter(|gate| gate.output == cure.0)
                .for_each(|gate| {
                    gate.output = "*****".to_string();
                });
        });
        input.gates.values_mut().for_each(|gates| {
            gates
                .iter_mut()
                .filter(|gate| gate.output == cure.1)
                .for_each(|gate| {
                    gate.output = cure.0.to_string();
                });
        });
        input.gates.values_mut().for_each(|gates| {
            gates
                .iter_mut()
                .filter(|gate| gate.output == "*****")
                .for_each(|gate| {
                    gate.output = cure.1.to_string();
                });
        });
    });

    input
}

fn p2(input: &str) -> String {
    let input = cure_p2(Input::parse_input(input));

    let mut output = String::new();

    output.push_str("digraph G {\n");

    let mut number = 0;
    while input.wires.contains_key(&get_x(number)) {
        output.push_str(&format!(
            r#"  subgraph c_{} {{
    {} [style=filled,fillcolor=green];
    {} [style=filled,fillcolor=green];
    {}
  }}
"#,
            number,
            get_x(number),
            get_y(number),
            get_z(number),
        ));
        number += 1;
    }

    /*
    let max_number = number - 1;

    (0..max_number).for_each(|wire| {
        output.push_str(&format!(
            "  {} [style=filled,fillcolor=blue];\n",
            get_z(wire)
        ));
    });
    */

    let rev_gates =
        input
            .gates
            .values()
            .fold(HashMap::new(), |mut acc: HashMap<String, Gate>, gates| {
                gates.iter().for_each(|gate| {
                    if !acc.contains_key(&gate.output) {
                        acc.insert(gate.output.clone(), gate.clone());
                    }
                });
                acc
            });

    rev_gates.values().for_each(|gate| {
        let out_color = match gate.operator {
            Operator::And => "red",
            Operator::Or => "yellow",
            Operator::Xor => "blue",
        };

        output.push_str(&format!(
            "  {} [style=filled,fillcolor={}];\n",
            gate.output, out_color
        ));

        output.push_str(&format!("  {} -> {}\n", gate.inputs[0], gate.output));
        output.push_str(&format!("  {} -> {}\n", gate.inputs[1], gate.output));
    });

    output.push_str("}\n");

    output
}

fn main() {
    // disable p1 for this day, so that p2 output can be piped to a file immediately
    //println!("{}", p1(ACTUAL_INPUT));
    let _ = p1(ACTUAL_INPUT);

    // how to use this output:
    //      - prerequisite: `sudo apt install graphviz`
    //      - 1: `cd src/bin/day24/`
    //      - 2: `cargo r --bin day24 > graph.dot`
    //      - 3: `dot -Tsvg graph.dot > output.svg`
    //      - 4: View file in SVG viewer
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = r"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const SAMPLE_INPUT_2: &str = r"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT_1), "4");
        assert_eq!(p1(SAMPLE_INPUT_2), "2024");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "51715173446832");
    }

    #[test]
    fn test_p2_actual() {
        let mut cure_values = P2_CURE
            .iter()
            .flat_map(|vals| vec![vals.0, vals.1])
            .collect::<Vec<_>>();
        cure_values.sort_unstable();

        assert_eq!(cure_values.join(","), "dpg,kmb,mmf,tvp,vdk,z10,z15,z25");
    }
}
