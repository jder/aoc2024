use euclid::point2;
use log::debug;
use regex::Regex;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let integer = Regex::new("-?\\d+").unwrap();
    let duration = 100;
    let robots = input.lines().map(|line| {
        let (px, py, vx, vy) = integer
            .find_iter(line)
            .map(|x| x.as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (px + vx * duration, py + vy * duration)
    });

    let width = 101;
    let height = 103;

    let final_positions = robots.map(|(x, y)| (x.rem_euclid(width), y.rem_euclid(height)));
    let quadrants = final_positions
        .filter(|(x, y)| *x != width / 2 && *y != height / 2)
        .into_grouping_map_by(|(x, y)| (*x < width / 2, *y < height / 2))
        .fold(0, |acc, _key, _val| acc + 1);

    quadrants.values().product()
}

pub fn part2(input: &str) -> usize {
    let integer = Regex::new("-?\\d+").unwrap();
    let robots = input.lines().map(|line| {
        let (px, py, vx, vy) = integer
            .find_iter(line)
            .map(|x| x.as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (px, py, vx, vy)
    });

    let width = 101;
    let height = 103;

    let result = (0..((width * height) as i64))
        .map(|i| {
            let grid = build_grid(width, height, robots.clone(), i);

            let num_well_connected = grid
                .cells()
                .filter(|cell| {
                    *cell.contents() == '#'
                        && cell
                            .cardinal_neighbors()
                            .filter(|c| *c.contents() == '#')
                            .count()
                            >= 2
                })
                .count();
            (i, num_well_connected)
        })
        .sorted_by(|(_, a), (_, b)| a.cmp(b).reverse())
        .next()
        .unwrap()
        .0 as usize;

    let final_grid = build_grid(width, height, robots, result as i64);
    for y in 0..(height as Index) {
        let line = (0..(width as Index))
            .map(|x| {
                final_grid
                    .cell(point2(x, y))
                    .unwrap()
                    .contents()
                    .to_string()
            })
            .join("");
        debug!("{}", line);
    }

    result
}

fn build_grid(
    width: usize,
    height: usize,
    robots: impl Iterator<Item = (i64, i64, i64, i64)>,
    i: i64,
) -> Grid<char> {
    let mut grid = Grid::new('.', width, height);
    for pos in robots.map(|(x, y, vx, vy)| {
        (
            (x + vx * i).rem_euclid(width as i64),
            (y + vy * i).rem_euclid(height as i64),
        )
    }) {
        grid.set(point2(pos.0 as Index, pos.1 as Index), '#');
    }
    grid
}
