use std::iter;

use itertools::Either;

use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let map = Grid::new_with_lines(input.lines());
    let stations = map.cells().filter(|c| *c.contents() != '.');

    stations
        .clone()
        .cartesian_product(stations)
        .filter_map(|(a, b)| {
            if a != b && a.contents() == b.contents() {
                let a_to_b = b.location() - a.location();
                let twice_b = a.location() + a_to_b * 2;
                map.cell(twice_b)
            } else {
                None
            }
        })
        .unique()
        .count()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let map = &Grid::new_with_lines(input.lines());
    let stations = map.cells().filter(|c| *c.contents() != '.');

    stations
        .clone()
        .cartesian_product(stations)
        .flat_map(|(a, b)| {
            if a != b && a.contents() == b.contents() {
                let a_to_b = b.location() - a.location();
                let mut now = a.location();
                Either::Left(iter::from_fn(move || {
                    now += a_to_b;
                    map.cell(now)
                }))
            } else {
                Either::Right(iter::empty())
            }
        })
        .unique()
        .count()
}
