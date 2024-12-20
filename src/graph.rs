use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    iter,
};

use itertools::Itertools;

pub fn find<V, EdgeIterator>(
    start: V,
    mut edges: impl FnMut(V) -> EdgeIterator,
    mut predicate: impl FnMut(&V) -> bool,
) -> impl Iterator<Item = V>
where
    EdgeIterator: Iterator<Item = V>,
    V: Eq + Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut queue = vec![start];

    std::iter::from_fn(move || {
        while let Some(node) = queue.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());
            queue.extend(edges(node.clone()));

            if predicate(&node) {
                return Some(node);
            }
        }
        None
    })
}

pub fn min_distances<V>(start: V, edges: impl Fn(&V) -> Vec<(V, u64)>) -> HashMap<V, u64>
where
    V: Eq + Ord + Hash + Clone,
{
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));
    while let Some(Reverse((distance, node))) = queue.pop() {
        if let Some(&previous_distance) = distances.get(&node) {
            if distance >= previous_distance {
                continue;
            }
        }
        distances.insert(node.clone(), distance);
        for (next_node, next_distance) in edges(&node) {
            queue.push(Reverse((distance + next_distance, next_node)));
        }
    }
    distances.into_iter().collect()
}

pub fn all_paths<V>(start: V, end: V, edges: impl Fn(&V) -> Vec<V>) -> impl Iterator<Item = Vec<V>>
where
    V: Eq + Ord + Hash + Clone,
{
    let first_edges = edges(&start);
    let mut path_and_local_queues = vec![(start, first_edges)];
    return iter::from_fn(move || {
        while let Some((node, ref mut alternatives)) = path_and_local_queues.last_mut() {
            if *node == end {
                let node = node.clone();
                return Some(
                    path_and_local_queues
                        .iter()
                        .map(|(n, _)| n.clone())
                        .chain(iter::once(node))
                        .collect_vec(),
                );
            }

            if let Some(next_node) = alternatives.pop() {
                let next_edges = edges(&next_node);
                path_and_local_queues.push((next_node, next_edges));
            } else {
                path_and_local_queues.pop();
            }
        }

        None
    });
}

pub fn flood_fill_from<V, EI>(
    starts: impl Iterator<Item = V>,
    edges: impl Fn(&V) -> EI,
) -> Vec<Vec<V>>
where
    V: Eq + Hash + Clone,
    EI: Iterator<Item = V>,
{
    let mut regions = vec![];
    let mut visited: HashMap<V, usize> = HashMap::new();

    for start in starts {
        if visited.contains_key(&start) {
            continue;
        }
        let mut queue = vec![start];
        let mut region = vec![];
        while let Some(node) = queue.pop() {
            if visited.contains_key(&node) {
                continue;
            }
            visited.insert(node.clone(), regions.len());
            region.push(node.clone());
            for next_node in edges(&node) {
                queue.push(next_node);
            }
        }
        if !region.is_empty() {
            regions.push(region);
        }
    }

    regions
}
