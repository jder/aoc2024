use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> i32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .map(|(f, s)| (f - s).abs())
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> u64 {
    let (first, second): (Vec<u64>, HashBag<u64>) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    first
        .into_iter()
        .map(|x| x * second.get(&x).map(|(_, count)| count).unwrap_or_default() as u64)
        .sum()
}
