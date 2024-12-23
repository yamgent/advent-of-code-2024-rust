use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/23/input.txt");

fn p1(input: &str) -> String {
    let edges = input
        .trim()
        .lines()
        .map(|line| line.trim().split_once("-").expect("xx-xx"))
        .collect::<Vec<_>>();

    let graph = edges.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<&str, HashSet<&str>>, edge| {
            acc.entry(edge.0).or_default().insert(edge.1);
            acc.entry(edge.1).or_default().insert(edge.0);
            acc
        },
    );

    edges
        .iter()
        .fold(HashSet::new(), |mut acc, edge| {
            graph
                .get(edge.0)
                .expect("visited")
                .intersection(graph.get(edge.1).expect("visited"))
                .into_iter()
                .map(|third| {
                    let mut nodes = vec![edge.0, edge.1, third];
                    nodes.sort_unstable();
                    (nodes[0], nodes[1], nodes[2])
                })
                .filter(|group| {
                    group.0.starts_with("t") || group.1.starts_with("t") || group.2.starts_with("t")
                })
                .for_each(|group| {
                    acc.insert(group);
                });
            acc
        })
        .len()
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
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), "7");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "1046");
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
