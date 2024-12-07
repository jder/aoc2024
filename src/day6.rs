use bitvec::vec::BitVec;

use crate::prelude::*;

#[repr(isize)]
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn heading(&self) -> (Index, Index) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}

struct Visited {
    width: usize,
    bitvec: BitVec,
}

impl Visited {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            bitvec: BitVec::repeat(false, width * height * 4),
        }
    }

    fn insert(&mut self, location: Location, heading: Direction) -> bool {
        let index = (location.y as usize * self.width + location.x as usize) * 4 + heading as usize;
        let was_set = *self.bitvec.get(index).unwrap();
        self.bitvec.set(index, true);
        was_set
    }

    fn iter(&self) -> impl Iterator<Item = (Location, Direction)> + '_ {
        self.bitvec
            .iter()
            .enumerate()
            .filter_map(move |(index, set)| {
                if *set {
                    let y = index / (self.width * 4);
                    let x = (index % (self.width * 4)) / 4;
                    let heading = index % 4;
                    Some((
                        Location {
                            x: x as Index,
                            y: y as Index,
                        },
                        match heading {
                            0 => Direction::Up,
                            1 => Direction::Right,
                            2 => Direction::Down,
                            3 => Direction::Left,
                            _ => unreachable!(),
                        },
                    ))
                } else {
                    None
                }
            })
    }
}

fn walk(map: &Grid<char>, start: Location) -> Option<Visited> {
    let mut visited = Visited::new(map.width(), map.height());

    let mut now = map.cell(start).unwrap();
    let mut direction = Direction::Up;
    loop {
        if visited.insert(now.location(), direction) {
            break None;
        }
        if let Some(in_front) = now.offset(direction.heading().0, direction.heading().1) {
            if *in_front.contents() == '#' {
                // turn 90 degrees to the right
                direction = direction.turn_right();
            } else {
                now = in_front;
            }
        } else {
            break Some(visited);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let map = Grid::new_with_lines(input.lines());
    let start = map
        .cells()
        .find(|cell| *cell.contents() == '^')
        .unwrap()
        .location();
    walk(&map, start).unwrap().iter().unique_by(|v| v.0).count()
}

pub fn part2(input: &str) -> usize {
    let mut map = Grid::new_with_lines(input.lines());
    let start = map
        .cells()
        .find(|cell| *cell.contents() == '^')
        .unwrap()
        .location();

    let visited = walk(&map, start).unwrap();
    let possible_obstacles = visited
        .iter()
        .unique_by(|v| v.0)
        .map(|(location, _)| location);

    possible_obstacles
        .into_iter()
        .filter(|&location| {
            map.set(location, '#');
            let is_loop = walk(&map, start).is_none();
            map.set(location, '.');
            is_loop
        })
        .count()
}
