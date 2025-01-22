use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    character::complete::{self, alpha1, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let graph = parse(input)?;
    let k3s = graph.k3_subgraphs();
    Ok(k3s
        .iter()
        .filter(|k3| {
            k3.iter().any(|&computer| {
                let label = &graph.vertices[computer];
                candidate_historian_computer(label)
            })
        })
        .count())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let graph = parse(input)?;
    let maximal_clique = graph.maximal_cliques().into_iter().exactly_one()?;
    Ok(password(&graph, &maximal_clique))
}

fn candidate_historian_computer(computer: &str) -> bool {
    computer.starts_with('t')
}

fn password(graph: &UndirectedGraph, vertex_indices: &HashSet<usize>) -> String {
    vertex_indices
        .iter()
        .map(|index| &graph.vertices[*index])
        .sorted()
        .join(",")
}

type Multimap<K, V> = HashMap<K, HashSet<V>>;

struct UndirectedGraph {
    vertices: Vec<String>,
    neighbors: Multimap<usize, usize>,
}

impl UndirectedGraph {
    fn parse(input: &str) -> IResult<&str, UndirectedGraph> {
        map(
            all_consuming(separated_list1(
                line_ending,
                separated_pair(alpha1, complete::char('-'), alpha1),
            )),
            |edges: Vec<(&str, &str)>| {
                let vertices = edges
                    .iter()
                    .flat_map(|(a, b)| [a.to_string(), b.to_string()].into_iter())
                    .sorted()
                    .dedup()
                    .collect::<Vec<String>>();

                let mut neighbors = Multimap::new();
                for (a, b) in &edges {
                    let a_index = vertices.binary_search(&a.to_string()).unwrap();
                    let b_index = vertices.binary_search(&b.to_string()).unwrap();
                    neighbors.entry(a_index).or_default().insert(b_index);
                    neighbors.entry(b_index).or_default().insert(a_index);
                }

                UndirectedGraph {
                    vertices,
                    neighbors,
                }
            },
        )(input)
    }

    /// Returns all complete sub-graphs of degree 3.
    fn k3_subgraphs(&self) -> HashSet<[usize; 3]> {
        let mut k3s = HashSet::new();
        for x in 0..self.vertices.len() {
            for y in &self.neighbors[&x] {
                for z in &self.neighbors[y] {
                    if self.neighbors[z].contains(&x) {
                        let mut subgraph = [x, *y, *z];
                        subgraph.sort();
                        k3s.insert(subgraph);
                    }
                }
            }
        }

        k3s
    }

    /// Returns all complete sub-graphs of maximal size.
    fn maximal_cliques(&self) -> Vec<HashSet<usize>> {
        let mut cliques = Vec::new();
        self.bron_kerbosch(
            HashSet::new(),
            (0..self.vertices.len()).collect::<HashSet<usize>>(),
            HashSet::new(),
            &mut cliques,
        );
        cliques.into_iter().max_set_by(|a, b| a.len().cmp(&b.len()))
    }

    fn bron_kerbosch(
        &self,
        r: HashSet<usize>,
        mut p: HashSet<usize>,
        mut x: HashSet<usize>,
        cliques: &mut Vec<HashSet<usize>>,
    ) {
        if p.is_empty() && x.is_empty() {
            cliques.push(r);
            return;
        }

        let pivot = p.union(&x).next().unwrap();
        let candidates = p
            .difference(&self.neighbors[pivot])
            .copied()
            .collect::<Vec<usize>>();

        for v in candidates {
            let mut new_r = r.clone();
            new_r.insert(v);

            let new_p = p.intersection(&self.neighbors[&v]).copied().collect();
            let new_x = x.intersection(&self.neighbors[&v]).copied().collect();

            self.bron_kerbosch(new_r, new_p, new_x, cliques);

            p.remove(&v);
            x.insert(v);
        }
    }
}

fn parse(input: &str) -> anyhow::Result<UndirectedGraph> {
    let (_, graph) =
        UndirectedGraph::parse(input).map_err(|e| anyhow!("Failed to parse input: {}", e))?;
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "kh-tc
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
td-yn";

    #[test]
    fn part1_returns_k3_graphs_with_t_computer() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 7);
        Ok(())
    }

    #[test]
    fn part2_todo() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, "co,de,ka,ta");
        Ok(())
    }
}
