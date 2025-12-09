use std::fs::read_to_string;

use itertools::Itertools;

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
    todo!()
}
