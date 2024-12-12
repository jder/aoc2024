use std::collections::HashSet;

use graph::flood_fill_from;

use crate::prelude::*;

pub fn compute_cost(
    input: &str,
    fence_metric: impl for<'a> Fn(&Grid<char>, &[Cell<'a, char>]) -> usize,
) -> usize {
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
            let fence_cost = fence_metric(&map, region);
            area * fence_cost
        })
        .sum()
}

fn fence_segments<'a>(cell: &'a Cell<char>) -> impl Iterator<Item = Face> + 'a {
    cardinal_headings().filter_map(|heading| match cell.offset(heading.0, heading.1) {
        Some(c) if c.contents() != cell.contents() => Some(Face::new(cell.location(), heading)),
        None => Some(Face::new(cell.location(), heading)),
        _ => None,
    })
}

pub fn part1(input: &str) -> usize {
    compute_cost(input, |_map, region| {
        region.iter().map(|cell| fence_segments(cell).count()).sum()
    })
}

// of the two sides of this face (as returned by Face::touching_locations) which of them matches the given character?
fn side_matching_character(map: &Grid<char>, face: &Face, c: char) -> usize {
    face.touching_locations()
        .find_position(|&loc| {
            map.cell(loc)
                .map(|cell| *cell.contents() == c)
                .unwrap_or(false)
        })
        .unwrap()
        .0
}

pub fn part2(input: &str) -> usize {
    compute_cost(input, |map, region| {
        let region_character = *region.first().unwrap().contents();
        let segments: &HashSet<Face> = &region.iter().flat_map(fence_segments).collect();
        flood_fill_from(segments.iter().copied(), move |face| {
            let needed_side = side_matching_character(map, face, region_character);
            face.same_direction_neighbors()
                .filter(move |maybe_neighbor| {
                    segments.contains(maybe_neighbor)
                        && side_matching_character(map, maybe_neighbor, region_character)
                            == needed_side
                })
        })
        .len()
    })
}
