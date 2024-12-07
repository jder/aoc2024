use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let map = Grid::new_with_lines(input.lines());
    let mut visited = Region::new();

    let mut now = map.cells().find(|cell| *cell.contents() == '^').unwrap();
    let mut heading = (0, -1);
    loop {
        visited.insert(now.location());
        if let Some(in_front) = now.offset(heading.0, heading.1) {
            if *in_front.contents() == '#' {
                // turn 90 degrees to the right
                heading = (-heading.1, heading.0);
            } else {
                now = in_front;
            }
        } else {
            break;
        }
    }

    visited.len()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
