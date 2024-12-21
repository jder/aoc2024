use std::iter;

use log::debug;

use crate::prelude::*;

struct Keypad {
    map: Grid<char>,
}

impl Keypad {
    fn new(keypad: &str) -> Self {
        let map = Grid::new_with_lines(keypad.lines());
        Self { map }
    }

    fn paths(&self, from: char, to: char) -> impl Iterator<Item = Vec<char>> {
        let from_cell = self.map.cells().find(|c| *c.contents() == from).unwrap();
        let to = self
            .map
            .cells()
            .find(|c| *c.contents() == to)
            .unwrap()
            .location();

        let mut now = from_cell.location();

        let mut headings = Vec::new();
        while now != to {
            if now.x != to.x {
                let delta = (to.x - now.x).signum();
                headings.push(vec2(delta, 0));
                now.x += delta;
                continue;
            }

            if now.y != to.y {
                let delta = (to.y - now.y).signum();
                headings.push(vec2(0, delta));
                now.y += delta;
                continue;
            }
        }

        [
            self.to_path(from_cell, headings.iter().copied()),
            self.to_path(from_cell, headings.iter().copied().rev()),
        ]
        .into_iter()
        .flatten()
    }

    fn to_path(
        &self,
        from: Cell<char>,
        headings: impl Iterator<Item = Heading>,
    ) -> Option<Vec<char>> {
        let mut now = from;
        let mut path = Vec::new();
        for heading in headings {
            now = now.offset(heading.x, heading.y).unwrap();
            if *now.contents() == '#' {
                return None;
            }
            path.push(if heading == NORTH {
                '^'
            } else if heading == SOUTH {
                'v'
            } else if heading == EAST {
                '>'
            } else {
                '<'
            });
        }

        Some(path)
    }
}

static FINAL_KEYPAD: &'static str = "789
456
123
#0A";

static ARROW_KEYPAD: &'static str = "#^A
<v>";

// Given the previously-pressed button, find min number of moves to *press* target
fn best_path_to(
    from: char,
    to: char,
    keypads: &[&Keypad],
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if keypads.is_empty() {
        return 1;
    }

    if let Some(&cost) = cache.get(&(from, to, keypads.len())) {
        return cost;
    }

    let possible_paths = keypads.first().unwrap().paths(from, to);
    let cost = possible_paths
        .into_iter()
        .map(|path| {
            path.into_iter()
                .chain(iter::once('A'))
                .fold(('A', 0), |(last, cost), c| {
                    let new_cost = cost + best_path_to(last, c, &keypads[1..], cache);
                    (c, new_cost)
                })
                .1
        })
        .min()
        .unwrap();

    cache.insert((from, to, keypads.len()), cost);
    cost
}

fn solve(
    target: &str,
    final_keypad: &Keypad,
    arrow_keypad: &Keypad,
    num_arrow_pads: usize,
) -> usize {
    let mut cache = HashMap::new();
    let keypads = iter::once(final_keypad)
        .chain(iter::repeat(arrow_keypad).take(num_arrow_pads))
        .collect::<Vec<_>>();
    let final_cost: usize = target
        .chars()
        .fold(('A', 0), |(from, total), to| {
            (
                to,
                total + best_path_to(from, to, &keypads.as_slice(), &mut cache),
            )
        })
        .1;

    debug!("{}: {}", target, final_cost);

    target[..3].parse::<usize>().unwrap() * final_cost
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let final_keypad = Keypad::new(FINAL_KEYPAD);
    let arrow_keypad = Keypad::new(ARROW_KEYPAD);

    input
        .lines()
        .map(|line| solve(line, &final_keypad, &arrow_keypad, 2))
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let final_keypad = Keypad::new(FINAL_KEYPAD);
    let arrow_keypad = Keypad::new(ARROW_KEYPAD);

    input
        .lines()
        .map(|line| solve(line, &final_keypad, &arrow_keypad, 25))
        .sum()
}
