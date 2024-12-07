use crate::prelude::*;

type Num = u64;
fn reachable(goal: Num, operands: &[Num], allow_concat: bool) -> bool {
    if operands.is_empty() {
        return false;
    }

    if operands == &[goal] {
        return true;
    }

    let (&last, rest) = operands.split_last().unwrap();

    if last > goal {
        return false;
    }

    if let (new_goal, 0) = goal.div_rem(&last) {
        if reachable(new_goal, rest, allow_concat) {
            return true;
        }
    }

    if allow_concat {
        let digits = last.checked_ilog10().unwrap_or(0) + 1;
        let (new_goal, needed_last) = goal.div_rem(&10u64.pow(digits));
        if needed_last == last && reachable(new_goal, rest, allow_concat) {
            return true;
        }
    }

    reachable(goal - last, rest, allow_concat)
}

fn parse_line(line: &str) -> (Num, Vec<Num>) {
    let (goal, rest) = line.split_once(":").unwrap();
    let goal: Num = goal.parse().unwrap();
    let operands = rest
        .split_whitespace()
        .map(|o| o.parse().unwrap())
        .collect();
    (goal, operands)
}

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .filter_map(|line| {
            let (goal, operands) = parse_line(line);
            reachable(goal, &operands, false).then_some(goal)
        })
        .sum()
}

pub fn part2(input: &str) -> Num {
    input
        .lines()
        .filter_map(|line| {
            let (goal, operands) = parse_line(line);
            reachable(goal, &operands, true).then_some(goal)
        })
        .sum()
}
