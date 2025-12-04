use itertools::Itertools;
use std::fs::read_to_string;

#[allow(dead_code)]
mod aoc_lib;

#[cfg(test)]
mod tests;

type AnswerType = usize;

fn main() {
    let args = aoc_lib::Args::get();

    let content = args
        .path
        .map(|p| read_to_string(&p).expect("input: Could not open file"));

    if args.part1 {
        let now = std::time::Instant::now();
        println!(
            "Part 1: {}, found in {}ms",
            part1(content.as_deref().expect("No input file was opened")),
            now.elapsed().as_micros() as f32 / 1000.0
        );
    }

    if args.part2 {
        let now = std::time::Instant::now();
        println!(
            "Part 2: {}, found in {}ms",
            part2(content.as_deref().expect("No input file was opened")),
            now.elapsed().as_micros() as f32 / 1000.0
        );
    }
}

fn count_adjacent_rols(grid: &[Vec<u8>], row: usize, col: usize) -> AnswerType {
    let row_min = row.saturating_sub(1);
    let row_max = (row + 1).min(grid.len() - 1);
    let col_min = col.saturating_sub(1);
    let col_max = (col + 1).min(grid[0].len() - 1);

    (row_min..=row_max)
        .flat_map(|r| (col_min..=col_max).map(move |c| grid[r][c] == b'@'))
        .filter(|b| *b)
        .count()
        - 1
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.as_bytes().to_owned())
        .collect_vec()
}

fn part1(_input: &str) -> AnswerType {
    let grid = parse(_input);
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == b'@' && count_adjacent_rols(&grid, row, col) < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(_input: &str) -> AnswerType {
    let mut grid = parse(_input);
    let mut count = 0;
    let mut removed = false;

    loop {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == b'@'
                    && count_adjacent_rols(&grid, row, col) < 4 {
                        count += 1;
                        grid[row][col] = b'.';
                        removed = true;
                    }
            }
        }

        if !removed { break } else { removed = false }
    }

    count
}
