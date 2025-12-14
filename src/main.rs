use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;
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

fn part1(_input: &str) -> AnswerType {
    _input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (
                x.parse::<AnswerType>().unwrap(),
                y.parse::<AnswerType>().unwrap(),
            )
        })
        .combinations(2)
        .map(|combo| (combo[0].0.abs_diff(combo[1].0) + 1) * (combo[0].1.abs_diff(combo[1].1) + 1))
        .max()
        .unwrap()
}

fn part2(_input: &str) -> AnswerType {
    let points = _input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (
                x.parse::<AnswerType>().unwrap(),
                y.parse::<AnswerType>().unwrap(),
            )
        })
        .collect_vec();

    points
        .iter()
        .progress_with_style(ProgressStyle::default_bar())
        .combinations(2)
        .par_bridge()
        .map(|combo| {
            (
                (combo[0].0 + 1).min(combo[1].0 + 1),
                (combo[0].0 + 1).max(combo[1].0 + 1),
                (combo[0].1 + 1).min(combo[1].1 + 1),
                (combo[0].1 + 1).max(combo[1].1 + 1),
            )
        })
        .filter(|(x_min, x_max, y_min, y_max)| {
            for line in points.windows(2) {
                let (x_min2, x_max2) = if line[0].0 > line[1].0 {
                    (line[1].0, line[0].0)
                } else {
                    (line[0].0, line[1].0)
                };
                let (y_min2, y_max2) = if line[0].1 > line[1].1 {
                    (line[1].1, line[0].1)
                } else {
                    (line[0].1, line[1].1)
                };
                if x_min2 == x_max2
                    && (x_min + 1..=x_max - 1).contains(&x_min2)
                    && (y_min2..=y_max2).any(|y| (y_min + 1..y_max - 1).contains(&y))
                {
                    return false;
                }
                if y_min2 == y_max2
                    && (y_min + 1..=y_max - 1).contains(&y_min2)
                    && (x_min2..=x_max2).any(|x| (x_min + 1..x_max - 1).contains(&x))
                {
                    return false;
                }
            }
            true
        })
        .map(|(x_min, x_max, y_min, y_max)| (x_max - x_min + 1) * (y_max - y_min + 1))
        .max()
        .unwrap()
}
