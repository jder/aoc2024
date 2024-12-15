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
    let (map, directions) = input.split_once("\n\n").unwrap();

    let mut map = Grid::new_with_lines(map.lines());
    let directions = directions.chars().flat_map(heading);

    let mut robot = map
        .cells()
        .find(|c| *c.contents() == '@')
        .unwrap()
        .location();

    map.set(robot, '.');

    for d in directions {
        let next = robot + d;
        let next_content = *map.cell(next).unwrap().contents();
        robot = match next_content {
            '.' => next,
            'O' if try_shift_box(&mut map, next, d) => next,
            _ => robot,
        }
    }

    map.cells()
        .filter(|c| *c.contents() == 'O')
        .map(|c| c.location().x + c.location().y * 100)
        .sum::<isize>() as usize
}

fn try_shift_box(map: &mut Grid<char>, start: Point2D<Index>, direction: Vector2D<Index>) -> bool {
    let next = start + direction;
    let next_content = *map.cell(next).unwrap().contents();
    let result = match next_content {
        '.' => true,
        '#' => false,
        'O' => try_shift_box(map, next, direction),
        _ => unreachable!(),
    };

    if result {
        map.set(next, 'O');
        map.set(start, '.');
    }
    result
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
