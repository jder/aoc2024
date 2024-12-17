use std::cmp::Ordering;

use graph::min_distances;

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

    fn next_states(&self) -> Vec<(Self, u64)> {
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

        if let Some(c) = self.cell.offset(self.heading.x, self.heading.y)
            && *c.contents() != '#'
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

    let distances = min_distances(State::new(start, EAST), |s| s.next_states());

    *distances
        .iter()
        .filter(|(s, _)| s.cell == end)
        .map(|(_, d)| d)
        .min()
        .unwrap()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
