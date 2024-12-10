use std::collections::HashMap;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let map = parse_map(input);
    map.cells()
        .filter(|c| *c.contents() == 0)
        .map(|trail_head: Cell<'_, u32>| {
            graph::find(
                trail_head,
                |cell| {
                    let next = cell.contents() + 1;
                    cell.cardinal_neighbors()
                        .filter(move |n| *n.contents() == next)
                },
                |cell| *cell.contents() == 9,
            )
            .count()
        })
        .sum()
}

fn parse_map(input: &str) -> Grid<u32> {
    Grid::new_with_lines(input.lines()).map(|c| c.contents().to_digit(10).unwrap())
}

pub fn part2(input: &str) -> usize {
    let map = parse_map(input);

    let mut paths_from: HashMap<Location, usize> = HashMap::new();

    map.cells()
        .filter(|c| *c.contents() == 0)
        .map(|trail_head| get_paths(&map, trail_head, &mut paths_from))
        .sum()
}

fn get_paths(
    map: &Grid<u32>,
    cell: Cell<'_, u32>,
    paths_from: &mut HashMap<Location, usize>,
) -> usize {
    if let Some(&paths) = paths_from.get(&cell.location()) {
        return paths;
    }

    let result = {
        if *cell.contents() == 9 {
            1
        } else {
            let next = cell.contents() + 1;
            cell.cardinal_neighbors()
                .filter(|n| *n.contents() == next)
                .map(|n| get_paths(map, n, paths_from))
                .sum()
        }
    };

    paths_from.insert(cell.location(), result);
    result
}
