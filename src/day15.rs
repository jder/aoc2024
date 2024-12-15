use std::collections::HashSet;

use log::debug;

use crate::prelude::*;

fn heading(c: char) -> Option<Vector2D<Index>> {
    match c {
        '^' => Some(vec2(0, -1)),
        'v' => Some(vec2(0, 1)),
        '<' => Some(vec2(-1, 0)),
        '>' => Some(vec2(1, 0)),
        _ => None,
    }
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let (map, directions) = parse(input);
    solve(map, directions)
}

fn solve(mut map: Grid<char>, directions: impl Iterator<Item = Vector2D<isize>>) -> usize {
    let mut robot = map
        .cells()
        .find(|c| *c.contents() == '@')
        .unwrap()
        .location();

    map.set(robot, '.');

    for (i, d) in directions.enumerate() {
        let next = robot + d;
        let next_content = *map.cell(next).unwrap().contents();
        robot = match next_content {
            '.' => next,
            'O' | '[' | ']' if try_shift_box(&mut map, next, d) => next,
            _ => robot,
        };

        debug!("Step {}: {:?}\n{}\n\n", i, robot, map);
        map.cells().for_each(|c| match c.contents() {
            '[' => debug_assert!(*map.cell(c.location() + vec2(1, 0)).unwrap().contents() == ']'),
            ']' => debug_assert!(*map.cell(c.location() + vec2(-1, 0)).unwrap().contents() == '['),
            _ => {}
        });
    }

    map.cells()
        .filter(|c| *c.contents() == 'O' || *c.contents() == '[')
        .map(|c| c.location().x + c.location().y * 100)
        .sum::<isize>() as usize
}

fn parse(input: &str) -> (Grid<char>, impl Iterator<Item = Vector2D<Index>> + use<'_>) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let map = Grid::new_with_lines(map.lines());
    let directions = directions.chars().flat_map(heading);
    (map, directions)
}

// Returns a list of locations and desired new contents, in order they should be applied
fn can_shift_box(
    map: &Grid<char>,
    start: Point2D<Index>,
    direction: Vector2D<Index>,
) -> Option<Vec<(Location, char)>> {
    let current = *map.cell(start).unwrap().contents();

    let box_starts = match current {
        'O' => vec![start],
        '[' => vec![start, start + vec2(1, 0)],
        ']' => vec![start, start + vec2(-1, 0)],
        _ => unreachable!(),
    };

    let push_starts = match current {
        'O' => &box_starts,
        '[' | ']' if direction.x == 0 => &box_starts,
        '[' => {
            assert!(direction.x == 1);
            &box_starts[1..]
        }
        ']' => {
            assert!(direction.x == -1);
            &box_starts[1..]
        }
        _ => unreachable!(),
    };

    let results = push_starts
        .iter()
        .map(|&start| {
            let next = start + direction;
            let next_content = *map.cell(next).unwrap().contents();
            match next_content {
                '.' => Some(vec![]),
                '#' => None,
                'O' | '[' | ']' => can_shift_box(map, next, direction),
                _ => unreachable!(),
            }
        })
        .collect::<Option<Vec<Vec<(Location, char)>>>>()?;

    let our_moves = box_starts.iter().map(|&start| (start, '.')).chain(
        box_starts
            .iter()
            .map(|&start| (start + direction, *map.cell(start).unwrap().contents())),
    );

    Some(results.into_iter().flatten().chain(our_moves).collect())
}

fn try_shift_box(map: &mut Grid<char>, start: Point2D<Index>, direction: Vector2D<Index>) -> bool {
    if let Some(starts) = can_shift_box(map, start, direction) {
        let mut already_set = HashSet::new();
        for (loc, c) in starts {
            if !already_set.contains(&loc) {
                map.set(loc, c);
                // rather than track all the boxes as a whole to find the newly-vacated spaces,
                // we proactively set the spaces to empty in the list of moves. but other branches
                // in the list may have already set them to something else, so we need avoid clearing them.
                if c != '.' {
                    already_set.insert(loc);
                }
            }
        }
        true
    } else {
        false
    }
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let (map, directions) = parse(input);
    solve(double_width(map), directions)
}

fn double_width(map: Grid<char>) -> Grid<char> {
    let mut new_map = Grid::new('.', map.width() * 2, map.height());
    for y in 0..(map.height() as isize) {
        for x in 0..(map.width() as isize) {
            let (a, b) = match map.cell(point2(x, y)).unwrap().contents() {
                '@' => ('@', '.'),
                'O' => ('[', ']'),
                '#' => ('#', '#'),
                '.' => ('.', '.'),
                _ => unreachable!(),
            };

            new_map.set(point2(x * 2, y), a);
            new_map.set(point2(x * 2 + 1, y), b);
        }
    }
    new_map
}
