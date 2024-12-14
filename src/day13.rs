use crate::prelude::*;

type Num = i64;

#[derive(Debug)]
struct Game {
    goal: (Num, Num),
    a: (Num, Num),
    b: (Num, Num),
}

fn parse(input: &str) -> Vec<Game> {
    let integer = regex::Regex::new(r"(\d+)").unwrap();

    input
        .split("\n\n")
        .map(|game| {
            let (a, b, goal) = game
                .lines()
                .map(|line| {
                    integer
                        .find_iter(line)
                        .map(|x| x.as_str().parse::<Num>().unwrap())
                        .collect_tuple::<(Num, Num)>()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            Game { goal, a, b }
        })
        .collect()
}

fn solve_game(Game { goal, a, b }: Game) -> Option<Num> {
    let b_count = (goal.0 * a.1 - goal.1 * a.0) / (b.0 * a.1 - b.1 * a.0);
    let a_count = (goal.0 - b.0 * b_count) / a.0;

    if a_count * a.0 + b_count * b.0 == goal.0 && a_count * a.1 + b_count * b.1 == goal.1 {
        Some(3 * a_count + b_count)
    } else {
        None
    }
}

pub fn part1(input: &str, _is_sample: bool) -> Num {
    let games = parse(input);
    games.into_iter().filter_map(solve_game).sum()
}

pub fn part2(input: &str, _is_sample: bool) -> Num {
    let games = parse(input);
    games
        .into_iter()
        .flat_map(
            |Game {
                 goal: orig_goal,
                 a,
                 b,
             }| {
                solve_game(Game {
                    goal: (orig_goal.0 + 10000000000000, orig_goal.1 + 10000000000000),
                    a,
                    b,
                })
            },
        )
        .sum()
}
