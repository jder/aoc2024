use crate::prelude::*;

fn to_heights(grid: &Grid<char>) -> Vec<usize> {
    (0..grid.width())
        .map(|x| {
            grid.cell(point2(x as isize, 0))
                .unwrap()
                .walk_inclusive(0, 1)
                .filter(|c| *c.contents() == '#')
                .count()
                - 1
        })
        .collect_vec()
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let grids = input
        .split("\n\n")
        .map(|s| Grid::new_with_lines(s.lines()))
        .collect::<Vec<_>>();
    let grids_by_top_left = grids
        .iter()
        .into_group_map_by(|g| *g.cell(point2(0, 0)).unwrap().contents());
    let keys = grids_by_top_left[&'.']
        .iter()
        .map(|g| to_heights(g))
        .collect_vec();
    let locks = grids_by_top_left[&'#']
        .iter()
        .map(|g| to_heights(g))
        .collect_vec();

    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|(k, l)| k.iter().zip(l.iter()).all(|(k, l)| k + l <= 5))
        .count()
}

pub fn part2(_input: &str, _is_sample: bool) -> &'static str {
    "Merry Christmas!"
}
