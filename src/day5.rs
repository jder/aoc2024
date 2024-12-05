use std::collections::HashMap;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let (orderings, updates) = input.split_once("\n\n").unwrap();
    let mut followers: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in orderings.lines() {
        let (before, after) = line
            .split("|")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        followers.entry(before).or_default().push(after);
    }

    let updates = updates.lines().map(|line| {
        line.split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec()
    });

    updates
        .filter(|update| {
            update.is_sorted_by(|a, b| {
                !followers
                    .get(b)
                    .map(|followers| followers.contains(a))
                    .unwrap_or_default()
            })
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
