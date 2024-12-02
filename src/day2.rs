use crate::prelude::*;

fn report_safe(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels.iter().tuple_windows().map(|(a, b)| a - b).collect();
    diffs.iter().all(|diff| [-1, -2, -3].contains(diff))
        || diffs.iter().all(|diff| [1, 2, 3].contains(diff))
}

pub fn part1(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    reports.iter().filter(|levels| report_safe(&levels)).count()
}

pub fn part2(input: &str) -> i32 {
    todo!()
}
