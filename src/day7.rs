use num::Integer;

use crate::prelude::*;

type Num = u64;
fn reachable(goal: Num, operands: &[Num]) -> bool {
    if operands.is_empty() {
        return false;
    }

    if operands == &[goal] {
        return true;
    }

    let last = *operands.last().unwrap();

    if last > goal {
        return false;
    }

    if let (rest, 0) = goal.div_rem(&last) {
        if reachable(rest, &operands[..operands.len() - 1]) {
            return true;
        }
    }

    reachable(goal - last, &operands[..operands.len() - 1])
}

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .filter_map(|line| {
            let (goal, rest) = line.split_once(":").unwrap();
            let goal: Num = goal.parse().unwrap();
            let operands: Vec<Num> = rest
                .split_whitespace()
                .map(|o| o.parse().unwrap())
                .collect();

            reachable(goal, &operands).then_some(goal)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
