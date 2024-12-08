use std::iter;

use itertools::Either;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let map = Grid::new_with_lines(input.lines());
    map.cells()
        .filter(|c| *c.contents() != '.')
        .cartesian_product(map.cells())
        .filter_map(|(a, b)| {
            if a == b {
                return None;
            }

            if a.contents() == b.contents() {
                let a_to_b = b.location() - a.location();
                let twice_b = a.location() + a_to_b * 2;
                if map.cell(twice_b).is_some() {
                    return Some(twice_b);
                }
            }

            None
        })
        .unique()
        .count()
}

pub fn part2(input: &str) -> usize {
    let map = Grid::new_with_lines(input.lines());
    map.cells()
        .filter(|c| *c.contents() != '.')
        .cartesian_product(map.cells())
        .flat_map(|(a, b)| {
            if a == b {
                return Either::Left(iter::empty());
            }

            if a.contents() == b.contents() {
                let a_to_b = b.location() - a.location();
                let mut now = a.location();
                let map = &map;
                return Either::Right(iter::from_fn(move || {
                    now += a_to_b;
                    if map.cell(now).is_some() {
                        Some(now)
                    } else {
                        None
                    }
                }));
            }

            Either::Left(iter::empty())
        })
        .unique()
        .count()
}
