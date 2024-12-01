use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use grid::Region;

pub mod grid;

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

pub fn flood_fill_from<V>(
    starts: impl Iterator<Item = V>,
    edges: impl Fn(&V) -> Vec<V>,
) -> Vec<Vec<V>>
where
    V: Eq + Hash + Clone,
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
