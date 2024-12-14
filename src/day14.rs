use euclid::point2;
use log::debug;
use regex::Regex;

use crate::prelude::*;

pub fn part1(input: &str, is_sample: bool) -> usize {
    let (width, height) = if is_sample { (11, 7) } else { (101, 103) };

    let duration = 100;
    let final_positions = parse_robots(input).map(|(px, py, vx, vy)| {
        (
            (px + vx * duration).rem_euclid(width),
            (py + vy * duration).rem_euclid(height),
        )
    });

    let quadrants = final_positions
        .filter(|(x, y)| *x != width / 2 && *y != height / 2)
        .into_grouping_map_by(|(x, y)| (*x < width / 2, *y < height / 2))
        .fold(0, |acc, _key, _val| acc + 1);

    quadrants.values().product()
}

pub fn part2(input: &str, is_sample: bool) -> usize {
    assert!(!is_sample);

    let robots = parse_robots(input);

    let (width, height) = (101, 103);

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
        let line: String = (0..(width as Index))
            .map(|x| *final_grid.cell(point2(x, y)).unwrap().contents())
            .collect();
        debug!("{}", line);
    }

    result
}

fn parse_robots(input: &str) -> impl Iterator<Item = (i64, i64, i64, i64)> + Clone + use<'_> {
    let integer = Regex::new("-?\\d+").unwrap();
    let robots = input.lines().map(move |line| {
        let (px, py, vx, vy) = integer
            .find_iter(line)
            .map(|x| x.as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        (px, py, vx, vy)
    });
    robots
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
