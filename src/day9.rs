use std::iter;

use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
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

#[derive(Debug)]
struct File {
    number: usize,
    index: usize,
    len: usize,
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let mut files = Vec::new();
    let mut frees = Vec::new();

    let mut location = 0;
    for (index, mut chunk) in input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let file_len = chunk.next().unwrap();
        files.push(File {
            number: index,
            index: location,
            len: file_len,
        });
        location += file_len;

        if let Some(free_len) = chunk.next() {
            frees.push((location, free_len));
            location += free_len;
        }
    }

    for index in (0..files.len()).rev() {
        let len = files[index].len;
        let original_index = files[index].index;
        for (free_index, free_len) in &mut frees {
            if *free_index >= original_index {
                break;
            }
            if *free_len >= len {
                files[index].index = *free_index;
                *free_len -= len;
                *free_index += len;
                frees.push((original_index, len));
                break;
            }
        }
    }

    files
        .iter()
        .map(|f| (f.index..(f.index + f.len)).sum::<usize>() * f.number)
        .sum()
}
