use std::collections::BinaryHeap;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2024/23/input.txt");

struct Input<'a> {
    edges: Vec<(&'a str, &'a str)>,
    graph: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Input<'a> {
    fn parse(input: &'a str) -> Self {
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

        Self { edges, graph }
    }
}

fn p1(input: &str) -> String {
    let input = Input::parse(input);
    let edges = input.edges;
    let graph = input.graph;

    edges
        .iter()
        .fold(HashSet::new(), |mut acc, edge| {
            graph
                .get(edge.0)
                .expect("visited")
                .intersection(graph.get(edge.1).expect("visited"))
                .map(|third| {
                    let mut nodes = [edge.0, edge.1, third];
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
    let input = Input::parse(input);
    let graph = input.graph;

    // problem is akin to the "Clique problem", which is NP-Complete
    // so aim is to reduce the brute-force necessary
    // this algorithm is hopefully O(N * 2^13) [13 = number of out-edges]
    //
    graph
        .iter()
        .flat_map(|(node, neighbours)| {
            let neighbours = neighbours.iter().collect::<Vec<_>>();
            (0..(2usize.pow(neighbours.len() as u32)))
                .map(move |mut choice| {
                    let mut chosen = vec![*node];
                    let mut idx = 0;
                    while choice > 0 {
                        if choice % 2 == 1 {
                            chosen.push(neighbours[idx]);
                        }
                        choice /= 2;
                        idx += 1;
                    }
                    chosen.sort_unstable();
                    chosen
                })
                .filter(|group| {
                    group.iter().all(|node| {
                        group.iter().all(|node_b| {
                            node == node_b || graph.get(*node).expect("visited").contains(*node_b)
                        })
                    })
                })
                .map(|group| (group.len(), group))
        })
        .collect::<BinaryHeap<_>>()
        .pop()
        .expect("input to have an answer")
        .1
        .join(",")
}

// this is "impossible" to write without knowing the answer in the first
// place, because we have no way of knowing that the input is specially
// constructed in a way, such that the max clique size answer is always
// out_edges. If we know that, then the algorithm would be O(N * 13).
// But again, you can't reasonably know this until you have already solved
// day 23.
#[allow(dead_code)]
fn p2_cheat(input: &str) -> String {
    let input = Input::parse(input);
    let graph = input.graph;

    graph
        .iter()
        .flat_map(|(node, neighbours)| {
            neighbours
                .iter()
                .map(|delete| {
                    let mut chosen = neighbours
                        .iter()
                        .filter(|node| *node != delete)
                        .collect::<Vec<_>>();
                    chosen.push(node);
                    chosen.sort_unstable();
                    chosen
                })
                .filter(|group| {
                    group.iter().all(|node| {
                        group.iter().all(|node_b| {
                            node == node_b || graph.get(*node).expect("visited").contains(*node_b)
                        })
                    })
                })
        })
        .next()
        .expect("input to have an answer")
        .iter()
        .map(|x| **x)
        .collect::<Vec<_>>()
        .join(",")
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
        assert_eq!(p2(SAMPLE_INPUT), "co,de,ka,ta");
    }

    #[test]
    #[ignore = "np-complete problem, took 19s on local"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz");
    }

    #[test]
    fn test_p2_cheat_sample() {
        assert_eq!(p2_cheat(SAMPLE_INPUT), "co,de,ka,ta");
    }

    #[test]
    fn test_p2_cheat_actual() {
        assert_eq!(
            p2_cheat(ACTUAL_INPUT),
            "de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz"
        );
    }
}
