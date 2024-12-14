use crate::prelude::*;

type Num = u64;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    count_steps(input, 25)
}

fn count_steps(input: &str, steps: usize) -> usize {
    let numbers = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<Num>().unwrap())
        .collect::<Vec<_>>();

    let mut memo = HashMap::new();
    numbers
        .into_iter()
        .map(|n| num_stones(n, steps, &mut memo))
        .sum()
}

fn try_split(n: Num) -> Option<(Num, Num)> {
    let rendered = n.to_string();
    if rendered.len() % 2 == 0 {
        let half = rendered.len() / 2;
        let (left, right) = rendered.split_at(half);
        Some((left.parse().unwrap(), right.parse().unwrap()))
    } else {
        None
    }
}

fn num_stones(start: Num, steps: usize, memo: &mut HashMap<(Num, usize), usize>) -> usize {
    if steps == 0 {
        return 1;
    }

    if let Some(&result) = memo.get(&(start, steps)) {
        return result;
    }

    let result = if start == 0 {
        num_stones(1, steps - 1, memo)
    } else if let Some((a, b)) = try_split(start) {
        num_stones(a, steps - 1, memo) + num_stones(b, steps - 1, memo)
    } else {
        num_stones(start * 2024, steps - 1, memo)
    };

    memo.insert((start, steps), result);
    result
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    count_steps(input, 75)
}
