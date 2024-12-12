use graph::flood_fill_from;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let map = Grid::new_with_lines(input.lines());
    let regions = flood_fill_from(map.cells(), |cell| {
        let letter = *cell.contents();
        cell.cardinal_neighbors()
            .filter(move |c| *c.contents() == letter)
    });
    regions
        .iter()
        .map(|region| {
            let area = region.len();
            let perimeter: usize = region
                .iter()
                .map(|cell| {
                    cardinal_neighbors(cell.location())
                        .filter(|l| match map.cell(*l) {
                            Some(c) => !region.contains(&c),
                            None => true, // edge of map
                        })
                        .count()
                })
                .sum();
            area * perimeter
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
