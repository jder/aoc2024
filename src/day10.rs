use std::collections::HashMap;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let map = parse_map(input);
    map.cells()
        .filter(is_trail_head)
        .map(|trail_head: Cell<'_, u32>| graph::find(trail_head, next_steps, is_goal).count())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let map = parse_map(input);

    let mut paths_from: HashMap<Location, usize> = HashMap::new();

    map.cells()
        .filter(is_trail_head)
        .map(|trail_head| get_paths(&map, trail_head, &mut paths_from))
        .sum()
}

fn parse_map(input: &str) -> Grid<u32> {
    Grid::new_with_lines(input.lines()).map(|c| c.contents().to_digit(10).unwrap())
}

fn next_steps(cell: Cell<'_, u32>) -> impl Iterator<Item = Cell<'_, u32>> {
    let next = cell.contents() + 1;
    cell.cardinal_neighbors()
        .filter(move |n| *n.contents() == next)
}

fn is_trail_head(cell: &Cell<'_, u32>) -> bool {
    *cell.contents() == 0
}

fn is_goal(cell: &Cell<'_, u32>) -> bool {
    *cell.contents() == 9
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
        if is_goal(&cell) {
            1
        } else {
            next_steps(cell)
                .map(|n| get_paths(map, n, paths_from))
                .sum()
        }
    };

    paths_from.insert(cell.location(), result);
    result
}
