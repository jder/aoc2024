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
    let mut enabled = true;
    regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ if enabled => m
                .iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
                .product::<usize>(),
            _ => 0,
        })
        .sum()
}
