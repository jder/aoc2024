use std::{fmt::Display, path::Path};

use clap::Parser;

pub mod prelude {
    pub use super::graph::{flood_fill_from, min_distances};
    pub use super::grid::*;
    pub use hashbag::HashBag;
    pub use itertools::Itertools;
    pub use regex;
}

pub mod graph;
pub mod grid;

// Inspired by https://git.sr.ht/~gadanidis/aoc2024/tree/main/item/src/main.rs

type DayFn = Box<dyn Fn(&str) -> String + Send + Sync + 'static>;

struct Runner {
    days: Vec<(String, (DayFn, DayFn))>,
}
impl Runner {
    fn new() -> Self {
        Self { days: Vec::new() }
    }

    fn register_day<T1, T2, F1, F2>(&mut self, name: &str, part1: F1, part2: F2)
    where
        F1: Fn(&str) -> T1,
        F1: Send + Sync + 'static,
        T1: Display,
        F2: Fn(&str) -> T2,
        F2: Send + Sync + 'static,
        T2: Display,
    {
        self.days.push((
            name.to_string(),
            (
                Box::new(move |input| part1(input).to_string()),
                Box::new(move |input| part2(input).to_string()),
            ),
        ));
    }

    fn run(&self, day: &str, part: usize, sample: bool) {
        let (part1, part2) = &self
            .days
            .iter()
            .find(|(name, _)| name == day)
            .expect("Day not found")
            .1;

        let contents = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("input")
                .join(if sample {
                    format!("{}-sample", day)
                } else {
                    day.to_string()
                })
                .with_extension("txt"),
        )
        .expect("Failed to read input");
        let input = contents.trim();

        let result = match part {
            1 => part1(input),
            2 => part2(input),
            _ => panic!("Invalid part {}", part),
        };
        println!("{} part {}: {}", day, part, result);
    }

    fn run_all(&self) {
        for (day, _) in &self.days {
            self.run(day, 1, false);
            self.run(day, 2, false);
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Day to run (default all)
    day: Option<String>,

    /// Part to run (1 or 2) (default both)
    part: Option<usize>,

    /// Use sample data
    #[clap(long, short)]
    sample: bool,
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn main() {
    let mut runner = Runner::new();
    runner.register_day("day1", day1::part1, day1::part2);
    runner.register_day("day2", day2::part1, day2::part2);
    runner.register_day("day3", day3::part1, day3::part2);
    runner.register_day("day4", day4::part1, day4::part2);
    runner.register_day("day5", day5::part1, day5::part2);
    runner.register_day("day6", day6::part1, day6::part2);

    let args = Args::parse();

    if args.sample && args.day.is_none() {
        eprintln!("--sample requires --day");
        std::process::exit(1);
    }

    match args.day {
        Some(day) => match args.part {
            Some(part) => runner.run(&day, part, args.sample),
            None => {
                runner.run(&day, 1, args.sample);
                runner.run(&day, 2, args.sample);
            }
        },
        None => runner.run_all(),
    }
}
