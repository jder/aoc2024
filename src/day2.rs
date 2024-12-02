use std::iter;

use crate::prelude::*;

fn report_safe(levels: &[i32]) -> bool {
    let diffs: Vec<_> = levels.iter().tuple_windows().map(|(a, b)| a - b).collect();
    diffs.iter().all(|diff| [-1, -2, -3].contains(diff))
        || diffs.iter().all(|diff| [1, 2, 3].contains(diff))
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let reports = parse_reports(input);
    reports.iter().filter(|levels| report_safe(&levels)).count()
}

fn mutated_report_safe(levels: &[i32]) -> bool {
    iter::once(levels.to_vec())
        .chain((0..levels.len()).map(|i| {
            let mut levels = levels.to_vec();
            levels.remove(i);
            levels
        }))
        .any(|levels| report_safe(&levels))
}

pub fn part2(input: &str) -> usize {
    let reports = parse_reports(input);
    reports
        .iter()
        .filter(|levels| mutated_report_safe(&levels))
        .count()
}
