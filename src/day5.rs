use std::{cmp::Ordering, collections::HashMap};

use crate::prelude::*;

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (orderings, updates) = input.split_once("\n\n").unwrap();
    let mut followers: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in orderings.lines() {
        let (before, after) = line
            .split("|")
            .map(|num| num.parse().unwrap())
            .collect_tuple()
            .unwrap();

        followers.entry(before).or_default().push(after);
    }

    let updates = updates.lines().map(|line| {
        line.split(",")
            .map(|num| num.parse().unwrap())
            .collect_vec()
    });

    (followers, updates.collect())
}

fn compare(a: &usize, b: &usize, followers: &HashMap<usize, Vec<usize>>) -> Ordering {
    if followers
        .get(a)
        .map(|followers| followers.contains(b))
        .unwrap()
    {
        Ordering::Less
    } else {
        // Some sanity checks to make sure we have a total order
        assert!(a != b);
        assert!(followers
            .get(b)
            .map(|followers| followers.contains(a))
            .unwrap());
        Ordering::Greater
    }
}

pub fn part1(input: &str) -> usize {
    let (followers, updates) = parse_input(input);

    updates
        .into_iter()
        .filter(|update| update.is_sorted_by(|a, b| compare(a, b, &followers) == Ordering::Less))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (followers, updates) = parse_input(input);

    updates
        .into_iter()
        .filter(|update| !update.is_sorted_by(|a, b| compare(a, b, &followers) == Ordering::Less))
        .map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| compare(a, b, &followers));
            sorted[sorted.len() / 2]
        })
        .sum()
}
