use itertools::Itertools;
use std::fs;

fn main() {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = fs::read_to_string("input/day1.txt")
        .unwrap()
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

    let total = first
        .into_iter()
        .zip(second)
        .map(|(f, s)| (f - s).abs())
        .sum::<i32>();
    println!("{}", total);
}
