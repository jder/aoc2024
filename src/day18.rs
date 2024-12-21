use graph::min_distances;

use crate::prelude::*;

struct Puzzle {
    size: usize,
    coords: Vec<Location>,
}

impl Puzzle {
    fn new(is_sample: bool, input: &str) -> Self {
        let size = if is_sample { 7 } else { 71 };
        let coords = input
            .lines()
            .map(|l| {
                let coords: (isize, isize) = l
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Location::new(coords.0, coords.1)
            })
            .collect_vec();

        Self { size, coords }
    }

    fn end(&self) -> Location {
        point2((self.size - 1) as isize, (self.size - 1) as isize)
    }

    fn solve(&self, num_bytes: usize) -> Option<u64> {
        let mut grid = Grid::new(false, self.size, self.size);
        for coord in self.coords[..num_bytes].iter() {
            grid.set(*coord, true);
        }

        min_distances(grid.cell(point2(0, 0)).unwrap(), |cell| {
            cell.cardinal_neighbors()
                .filter(|n| !n.contents())
                .map(|c| (c, 1))
                .collect()
        })
        .get(&grid.cell(self.end()).unwrap())
        .copied()
    }
}

pub fn part1(input: &str, is_sample: bool) -> u64 {
    let puzzle = Puzzle::new(is_sample, input);
    let num_bytes = if is_sample { 12 } else { 1024 };

    puzzle.solve(num_bytes).unwrap()
}

pub fn part2(input: &str, is_sample: bool) -> String {
    let puzzle = Puzzle::new(is_sample, input);

    let failing_count = (0..=puzzle.coords.len())
        .into_iter()
        .collect_vec()
        .partition_point(|&n| puzzle.solve(n).is_some());
    let failing_byte = puzzle.coords[failing_count - 1];
    format!("{},{}", failing_byte.x, failing_byte.y)
}
