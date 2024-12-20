use graph::min_distances;

use crate::prelude::*;

struct Problem<'a> {
    pieces: &'a [Vec<char>],
    target: Vec<char>,
}

impl<'a> Problem<'a> {
    fn satisfiable(&self) -> bool {
        min_distances(Vec::new(), |so_far| {
            self.pieces
                .iter()
                .filter(|piece| self.usable(&so_far, piece))
                .map(|piece| (so_far.iter().chain(piece.iter()).copied().collect(), 0))
                .collect()
        })
        .contains_key(&self.target)
    }

    fn count_ways(&self) -> usize {
        let mut ways = vec![0; self.target.len() + 1];
        ways[0] = 1;
        for target_len in 0..=self.target.len() {
            for piece in self.pieces {
                let lookback = piece.len();
                if lookback > target_len {
                    continue;
                }

                if self.usable(&self.target[..(target_len - lookback)], piece) {
                    ways[target_len] += ways[target_len - lookback];
                }
            }
        }
        *ways.last().unwrap()
    }

    fn usable(&self, target_so_far: &[char], piece: &[char]) -> bool {
        self.target[target_so_far.len()..].starts_with(piece)
    }
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let (pieces, targets) = parse(input);

    targets
        .filter(|target| {
            Problem {
                pieces: &pieces,
                target: target.clone(),
            }
            .satisfiable()
        })
        .count()
}

fn parse(input: &str) -> (Vec<Vec<char>>, impl Iterator<Item = Vec<char>> + use<'_>) {
    let (pieces, targets) = input.split_once("\n\n").unwrap();
    let pieces = pieces
        .split(", ")
        .map(|piece| piece.chars().collect_vec())
        .collect_vec();
    let targets = targets.lines().map(|line| line.chars().collect_vec());
    (pieces, targets)
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let (pieces, targets) = parse(input);

    targets
        .map(|target| {
            Problem {
                pieces: &pieces,
                target: target.clone(),
            }
            .count_ways()
        })
        .sum()
}
