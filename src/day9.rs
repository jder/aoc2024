use std::iter;

use crate::prelude::*;

pub fn part1(input: &str) -> usize {
    let mut disk = Vec::new();

    for (index, mut chunk) in input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let file_len = chunk.next().unwrap();
        disk.extend(iter::repeat_n(Some(index), file_len));

        if let Some(free_len) = chunk.next() {
            disk.extend(iter::repeat_n(None, free_len));
        }
    }

    let mut next_free = 0;
    'outer: for index in (0..disk.len()).rev() {
        if let Some(file_no) = disk[index] {
            while disk[next_free].is_some() {
                next_free += 1;
                if next_free >= index {
                    break 'outer;
                }
            }
            disk[next_free] = Some(file_no);
            disk[index] = None;
        }
    }

    println!(
        "{:?}",
        disk.iter()
            .map(|d| if let Some(d) = d { *d as i32 } else { -1i32 })
            .collect::<Vec<_>>()
    );

    disk.iter()
        .enumerate()
        .map(
            |(index, file)| {
                if let Some(f) = file {
                    f * index
                } else {
                    0
                }
            },
        )
        .sum()
}

pub fn part2(input: &str) -> usize {
    todo!()
}
