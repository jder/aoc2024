use std::{iter, usize};

use crate::prelude::*;

fn mix_and_prune(secret: usize, x: usize) -> usize {
    (secret ^ x) % (1 << 24)
}

fn next(prev: usize) -> usize {
    let next = mix_and_prune(prev, prev * 64);
    let next = mix_and_prune(next, next / 32);
    mix_and_prune(next, next * 2048)
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    input
        .lines()
        .map(|line| {
            let secret = line.parse::<usize>().unwrap();
            iter::successors(Some(secret), |&secret| Some(next(secret)))
                .skip(2000)
                .next()
                .unwrap()
        })
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
