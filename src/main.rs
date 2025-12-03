use itertools::Itertools;
use std::fs::read_to_string;

#[allow(dead_code)]
mod aoc_lib;

#[cfg(test)]
mod tests;

type AnswerType = u64;

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

fn joltage(input: &str, battery_count: usize) -> AnswerType {
    use std::cmp::Ordering::*;
    input
        .lines()
        .map(|l| {
            let batteries = l
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect_vec();
            let mut total = 0;
            let mut max_pos = 0;
            for i in (0..battery_count).rev() {
                let (rel_max_pos, max) = batteries[max_pos..batteries.len() - i]
                    .iter()
                    .enumerate()
                    .max_by(|a, b| match a.1.cmp(b.1) {
                        Less | Equal => Less,
                        Greater => Greater,
                    })
                    .unwrap();
                total *= 10;
                total += max;
                max_pos += rel_max_pos + 1;
            }

            total
        })
        .sum()
}

fn part1(_input: &str) -> AnswerType {
    joltage(_input, 2)
}

fn part2(_input: &str) -> AnswerType {
    joltage(_input, 12)
}
