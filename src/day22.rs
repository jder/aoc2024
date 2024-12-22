use std::{collections::HashSet, iter, usize};

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

struct Deltas<const N: usize>([i8; N], usize);

impl<const N: usize> Deltas<N> {
    fn new() -> Self {
        Self([0; N], 0)
    }

    fn push(&mut self, delta: isize) {
        for i in 0..N - 1 {
            self.0[i] = self.0[i + 1];
        }
        self.0[N - 1] = delta as i8;
        self.1 += 1;
    }

    fn full(&self) -> bool {
        self.1 >= N
    }

    fn key(&self) -> [i8; N] {
        self.0
    }
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let mut bananas: HashMap<[i8; 4], usize> = HashMap::new();
    input.lines().for_each(|line| {
        let secret = line.parse::<usize>().unwrap();
        let mut deltas = Deltas::new();
        let mut seen = HashSet::new();
        iter::successors(Some(secret), |&secret| Some(next(secret)))
            .take(2001)
            .map(|secret| secret % 10)
            .reduce(|last, current| {
                deltas.push(current as isize - last as isize);
                if deltas.full() && !seen.contains(&deltas.key()) {
                    *bananas.entry(deltas.key()).or_default() += current;
                    seen.insert(deltas.key());
                }
                current
            });
    });
    bananas.values().copied().max().unwrap()
}
