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

fn part1(_input: &str) -> AnswerType {
    let (fresh_str, all_str) = _input
        .trim()
        .split_once(if cfg!(windows) { "\r\n\r\n" } else { "\n\n" })
        .unwrap();

    let ranges = fresh_str
        .lines()
        .map(|l| {
            let (min, max) = l.split_once('-').unwrap();
            min.parse::<AnswerType>().unwrap()..=max.parse::<AnswerType>().unwrap()
        })
        .collect_vec();

    all_str
        .lines()
        .map(|l| l.parse::<AnswerType>().unwrap())
        .filter(|n| {
            for range in &ranges {
                if range.contains(n) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part2(_input: &str) -> AnswerType {
    let (fresh_str, _) = _input
        .trim()
        .split_once(if cfg!(windows) { "\r\n\r\n" } else { "\n\n" })
        .unwrap();
    let mut ranges = fresh_str
        .lines()
        .map(|l| {
            let (min, max) = l.split_once('-').unwrap();
            let min = min.parse::<AnswerType>().unwrap();
            let max = max.parse::<AnswerType>().unwrap();
            min..=max
        })
        .collect_vec();

    let mut merged = true;
    while merged {
        merged = false;
        let mut aux: Vec<std::ops::RangeInclusive<AnswerType>> = Vec::with_capacity(ranges.len());
        'outer: for range in ranges.iter() {
            let min = *range.start();
            let max = *range.end();
            for aux_range in aux.iter_mut() {
                if *aux_range.start() <= min && max <= *aux_range.end() {
                    merged = true;
                    continue 'outer;
                }
                if *aux_range.start() >= min && max >= *aux_range.end() {
                    *aux_range = min..=max;
                    merged = true;
                    continue 'outer;
                }
                if *aux_range.end() >= min && max >= *aux_range.end() {
                    *aux_range = *aux_range.start()..=max;
                    merged = true;
                    continue 'outer;
                }
                if *aux_range.start() <= max && min <= *aux_range.start() && *aux_range.start() > 0
                {
                    *aux_range = min..=*aux_range.end();
                    merged = true;
                    continue 'outer;
                }
            }

            aux.push(min..=max);
        }
        ranges = aux;
    }

    ranges.into_iter().map(|r| r.count()).sum()
}
