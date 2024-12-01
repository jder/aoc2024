use hashbag::HashBag;
use itertools::Itertools;
use std::fs;

fn main() {
    let (first, second): (Vec<u64>, HashBag<u64>) = fs::read_to_string("input/day1.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    let total: u64 = first
        .into_iter()
        .map(|x| x * second.get(&x).map(|(_, count)| count).unwrap_or_default() as u64)
        .sum();
    println!("{}", total);
}
