use crate::prelude::*;

fn matches_any<'a>(candidates: &[&str], path: impl Iterator<Item = Cell<'a, char>>) -> bool {
    let path_str: String = path.map(|cell| *cell.contents()).collect();

    candidates.iter().any(|c| path_str.starts_with(c))
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let grid = Grid::new_with_lines(input.lines());
    grid.cells()
        .map(|cell| {
            all_headings()
                .filter(|heading| matches_any(&["XMAS"], cell.walk_inclusive(heading.0, heading.1)))
                .count()
        })
        .sum()
}

fn x_diagonals(
    center: Cell<char>,
) -> Option<(
    impl Iterator<Item = Cell<char>>,
    impl Iterator<Item = Cell<char>>,
)> {
    let diag_one = center.offset(-1, -1)?.walk_inclusive(1, 1);
    let diag_two = center.offset(1, -1)?.walk_inclusive(-1, 1);

    Some((diag_one, diag_two))
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let grid = Grid::new_with_lines(input.lines());
    grid.cells()
        .filter(|cell| {
            x_diagonals(*cell).is_some_and(|(diag1, diag2)| {
                matches_any(&["SAM", "MAS"], diag1) && matches_any(&["SAM", "MAS"], diag2)
            })
        })
        .count()
}
