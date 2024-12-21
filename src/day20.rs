use graph::min_distances;
use log::debug;

use crate::prelude::*;

fn edges<'a>(c: &Cell<'a, char>) -> Vec<(Cell<'a, char>, u64)> {
    c.cardinal_neighbors()
        .filter(|n| *n.contents() != '#')
        .map(|n| (n, 1))
        .collect()
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    solve(input, 2)
}

fn solve(input: &str, cheat_distance: usize) -> usize {
    let map = Grid::new_with_lines(input.lines());
    let start = map.cells().find(|c| *c.contents() == 'S').unwrap();
    let end = map.cells().find(|c| *c.contents() == 'E').unwrap();

    let distances_from_start = &min_distances(start, edges);
    let distances_to_end = &min_distances(end, edges);

    let no_cheating_cost = *distances_from_start
        .get(&end)
        .expect("no path from start to end");

    let possible_cheats = distances_from_start
        .into_iter()
        .flat_map(|(cheat_start, cost_to_cheat_start)| {
            distances_to_end
                .iter()
                .filter_map(move |(cheat_end, cost_to_cheat_end)| {
                    let needed_cheat_distance = cheat_start.manhattan_distance(cheat_end);
                    if needed_cheat_distance > cheat_distance {
                        return None;
                    } else {
                        Some(cost_to_cheat_start + cost_to_cheat_end + needed_cheat_distance as u64)
                    }
                })
        })
        .sorted()
        .collect_vec();

    debug!(
        "{:?}",
        possible_cheats
            .iter()
            .filter_map(|cost| cost.checked_sub(no_cheating_cost))
            .counts()
            .iter()
            .sorted()
    );

    possible_cheats
        .into_iter()
        .filter(|c| *c <= no_cheating_cost - 100)
        .count()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    solve(input, 20)
}
