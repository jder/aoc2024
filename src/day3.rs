use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    regex::Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|m| {
            m.iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
