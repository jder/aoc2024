use std::cmp::Ordering;

use graph::{flood_fill_from, min_distances};
use log::debug;

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone)]
struct State<'a> {
    cell: Cell<'a, char>,
    heading: Heading,
}

impl<'a> State<'a> {
    fn new(cell: Cell<'a, char>, heading: Heading) -> Self {
        Self { cell, heading }
    }

    fn next_states(&self, direction_of_time: isize) -> Vec<(Self, u64)> {
        let mut result = vec![
            (
                Self::new(self.cell.clone(), vec2(self.heading.y, -self.heading.x)),
                1000,
            ),
            (
                Self::new(self.cell.clone(), vec2(-self.heading.y, self.heading.x)),
                1000,
            ),
        ];

        if let Some(c) = self.cell.offset(
            direction_of_time * self.heading.x,
            direction_of_time * self.heading.y,
        ) && *c.contents() != '#'
        {
            result.push((Self::new(c, self.heading), 1));
        }

        result
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.cell
                .cmp(&other.cell)
                .then(self.heading.x.cmp(&other.heading.x))
                .then(self.heading.y.cmp(&other.heading.y)),
        )
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part1(input: &str, _is_sample: bool) -> u64 {
    let map = Grid::new_with_lines(input.lines());
    let start = map.cells().find(|c| *c.contents() == 'S').unwrap();
    let end = map.cells().find(|c| *c.contents() == 'E').unwrap();

    let distances = min_distances(State::new(start, EAST), |s| s.next_states(1));

    *distances
        .iter()
        .filter(|(s, _)| s.cell == end)
        .map(|(_, d)| d)
        .min()
        .unwrap()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let map = Grid::new_with_lines(input.lines());
    let start = map.cells().find(|c| *c.contents() == 'S').unwrap();
    let end = map.cells().find(|c| *c.contents() == 'E').unwrap();

    let distances = &min_distances(State::new(start, EAST), |s| s.next_states(1));
    let (min_state, min_cost) = distances
        .iter()
        .filter(|(s, _)| s.cell == end)
        .min_by_key(|(_, d)| *d)
        .unwrap();

    flood_fill_from(
        vec![(min_state.clone(), *min_cost)].into_iter(),
        |(state, cost_to_start)| {
            let cost_to_start = *cost_to_start;
            state
                .next_states(-1)
                .into_iter()
                .filter_map(move |(next_s, move_cost)| {
                    if cost_to_start == 0 {
                        return None;
                    }
                    debug!(
                        "cost_to_start: {}, move_cost: {}, distances[&next_s]: {}",
                        cost_to_start, move_cost, distances[&next_s]
                    );
                    if let Some(expected_cost) = cost_to_start.checked_sub(move_cost)
                        && distances[&next_s] == expected_cost
                    {
                        Some((next_s, cost_to_start - move_cost))
                    } else {
                        None
                    }
                })
        },
    )[0]
    .iter()
    .unique_by(|(s, _)| s.cell)
    .count()
}
