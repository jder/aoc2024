use std::iter;

use log::debug;

use crate::prelude::*;

struct Keypad {
    map: Grid<char>,
    order_matters: bool,
    enter_location: Location,
}

impl Keypad {
    fn new(keypad: &str, order_matters: bool) -> Self {
        let map = Grid::new_with_lines(keypad.lines());
        let enter_location = map
            .cells()
            .find(|c| *c.contents() == 'A')
            .unwrap()
            .location();
        Self {
            map,
            order_matters,
            enter_location,
        }
    }

    fn path(&mut self, location: Location, next_location: Location) -> impl Iterator<Item = char> {
        // Always move in Y first since that's guaranteed to be valid
        let mut moves = Vec::new();
        let mut now = self.map.cell(location).unwrap();
        let goal = self.map.cell(next_location).unwrap();

        while now != goal {
            if now.location().y < goal.location().y && *now.offset(0, 1).unwrap().contents() != '#'
            {
                moves.push('v');
                now = now.offset(0, 1).unwrap();
            }

            if now.location().y > goal.location().y && *now.offset(0, -1).unwrap().contents() != '#'
            {
                moves.push('^');
                now = now.offset(0, -1).unwrap();
            }

            if now.location().x < goal.location().x && *now.offset(1, 0).unwrap().contents() != '#'
            {
                moves.push('>');
                now = now.offset(1, 0).unwrap();
            }

            if now.location().x > goal.location().x && *now.offset(-1, 0).unwrap().contents() != '#'
            {
                moves.push('<');
                now = now.offset(-1, 0).unwrap();
            }
        }

        moves.into_iter()
    }
}

// ends in A, but not present in sequence
fn solve_block(sequence: impl Iterator<Item = char>, keypad: &mut Keypad) -> Vec<char> {
    sequence
        .chain(iter::once('A'))
        .fold(
            (keypad.enter_location, Vec::new()),
            |(location, mut moves), c| {
                let next_location = keypad
                    .map
                    .cells()
                    .find(|cell| *cell.contents() == c)
                    .unwrap()
                    .location();
                moves.extend(keypad.path(location, next_location));
                moves.extend(iter::once('A'));
                (next_location, moves)
            },
        )
        .1
}

// We assume we start at the enter location and the last move is to press the enter key
fn moves_to_enter(sequence: &[char], keypad: &mut Keypad) -> Vec<char> {
    let blocks = sequence.split(|c| *c == 'A');
    blocks
        .filter(|block| !block.is_empty())
        .flat_map(|block| solve_block(block.iter().copied(), keypad))
        .collect()
}

static FINAL_KEYPAD: &'static str = "789
456
123
#0A";

static ARROW_KEYPAD: &'static str = "#^A
<v>";

fn solve(target: &str, final_keypad: &mut Keypad, arrow_keypad: &mut Keypad) -> usize {
    let final_sequence = target.chars().collect_vec();
    let final_moves = moves_to_enter(&final_sequence, final_keypad);
    let robot_2_moves = moves_to_enter(&final_moves, arrow_keypad);
    let robot_1_moves = moves_to_enter(&robot_2_moves, arrow_keypad);

    debug!(
        "target: {}, final: {}, robot_2: {}, robot_1: {} length {}",
        target,
        final_moves.iter().collect::<String>(),
        robot_2_moves.iter().collect::<String>(),
        robot_1_moves.iter().collect::<String>(),
        robot_1_moves.len()
    );

    target[..3].parse::<usize>().unwrap() * robot_1_moves.len()
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let mut final_keypad = Keypad::new(FINAL_KEYPAD, true);
    let mut arrow_keypad = Keypad::new(ARROW_KEYPAD, false);

    input
        .lines()
        .map(|line| solve(line, &mut final_keypad, &mut arrow_keypad))
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
