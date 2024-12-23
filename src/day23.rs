use std::iter;

use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let nodes = &parse(input);

    nodes
        .iter()
        .filter(|(n, _)| n.starts_with("t"))
        .flat_map(|(node, connected)| {
            connected
                .iter()
                .cartesian_product(connected.iter())
                .filter_map(move |(a, b)| {
                    if nodes[a].contains(b) {
                        Some([node, a, b].into_iter().sorted().collect_vec())
                    } else {
                        None
                    }
                })
        })
        .unique()
        .count()
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    let pairs = input
        .lines()
        .map(|line| line.split("-").collect_tuple().unwrap());
    for (a, b) in pairs {
        nodes.entry(a).or_default().push(b);
        nodes.entry(b).or_default().push(a);
    }
    nodes
}

fn largest_component_with<'n>(
    nodes: &HashMap<&'n str, Vec<&'n str>>,
    so_far: Vec<&'n str>,
    candidates: &[&'n str],
) -> Vec<&'n str> {
    candidates
        .iter()
        .enumerate()
        .filter_map(|(i, candidate)| {
            if so_far
                .iter()
                .all(|current| nodes[current].contains(candidate))
            {
                let next = so_far
                    .iter()
                    .chain(iter::once(candidate))
                    .copied()
                    .collect_vec();
                Some(largest_component_with(nodes, next, &candidates[i + 1..]))
            } else {
                None
            }
        })
        .max_by_key(|c| c.len())
        .unwrap_or(so_far)
}

pub fn part2(input: &str, _is_sample: bool) -> String {
    let nodes = &parse(input);
    let mut largest = nodes
        .keys()
        .map(|n| largest_component_with(nodes, vec![n], &nodes[n]))
        .max_by_key(|n| n.len())
        .unwrap();

    largest.sort();
    largest.iter().join(",")
}
