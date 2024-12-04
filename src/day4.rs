use crate::prelude::*;

fn is_xmas<'a>(path: impl Iterator<Item = Cell<'a, char>>) -> bool {
    let xmas = "XMAS";
    path.take(xmas.len())
        .map(|cell| *cell.contents().unwrap())
        .eq(xmas.chars())
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::new_with_lines(input.lines());
    grid.cells()
        .map(|cell| {
            all_headings()
                .filter(|heading| is_xmas(cell.walk_inclusive(heading.0, heading.1)))
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
