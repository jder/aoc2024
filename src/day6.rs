use std::collections::{hash_set, HashSet};

use crate::prelude::*;

pub fn walk(map: &Grid<char>) -> Option<usize> {
    let mut visited = HashSet::new();

    let mut now = map.cells().find(|cell| *cell.contents() == '^').unwrap();
    let mut heading = (0, -1);
    loop {
        let current_state = (now.location(), heading);
        match visited.entry(current_state) {
            hash_set::Entry::Occupied(_) => break None,
            hash_set::Entry::Vacant(entry) => entry.insert(),
        }
        if let Some(in_front) = now.offset(heading.0, heading.1) {
            if *in_front.contents() == '#' {
                // turn 90 degrees to the right
                heading = (-heading.1, heading.0);
            } else {
                now = in_front;
            }
        } else {
            break Some(visited.into_iter().unique_by(|v| v.0).count());
        }
    }
}

pub fn part1(input: &str) -> usize {
    let map = Grid::new_with_lines(input.lines());
    walk(&map).unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut map = Grid::new_with_lines(input.lines());

    let possible_obstacles = map
        .cells()
        .filter_map(|cell| (*cell.contents() == '.').then_some(cell.location()))
        .collect_vec();

    possible_obstacles
        .into_iter()
        .filter(|&location| {
            map.set(location, '#');
            let is_loop = walk(&map).is_none();
            map.set(location, '.');
            is_loop
        })
        .count()
}
